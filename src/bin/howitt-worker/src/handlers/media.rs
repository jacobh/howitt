use howitt::ext::rayon::rayon_spawn_blocking;
use howitt::models::media::{ImageContentType, ImageSpec, Media, IMAGE_SPECS};
use howitt::repos::Repos;
use howitt::services::media::keys::{generate_resized_media_key, GenerateResizedMediaKeyParams};
use howitt::services::media::MediaGeoInferrer;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, RgbImage};
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};
use libwebp_sys::WebPPreset;
use std::io::Cursor;
use thiserror::Error;
use webp::WebPConfig;

use tracing::info;

use howitt::jobs::media::MediaJob;
use howitt_client_types::{BucketClient, ObjectParams};

use crate::context::Context;

fn resize(img: &DynamicImage, spec: &ImageSpec) -> DynamicImage {
    match spec {
        ImageSpec::Fill(dimensions) => {
            let (width, height) = dimensions.dimensions();

            img.resize_to_fill(width as u32, height as u32, FilterType::Lanczos3)
        }
        ImageSpec::Fit(dimensions) => {
            let (image_width, image_height) = img.dimensions();
            let (width, height) = dimensions.dimensions();

            if image_width <= width as u32 && image_height <= height as u32 {
                img.clone()
            } else {
                img.resize(width as u32, height as u32, FilterType::Lanczos3)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("Failed to encode JPEG: {0}")]
    JpegEncode(#[from] image::error::ImageError),
    #[error("Failed to create WebP encoder: {0}")]
    WebPEncoderCreation(String),
    #[error("Failed to encode WebP: {0}")]
    WebPEncode(String),
}

fn encode(img: &DynamicImage, content_type: &ImageContentType) -> Result<Vec<u8>, EncodeError> {
    match content_type {
        ImageContentType::Jpeg => {
            let mut buffer = Vec::new();
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 85);
            encoder.encode_image(img)?;
            Ok(buffer)
        }
        ImageContentType::Webp => {
            let encoder = webp::Encoder::from_image(&img)
                .map_err(|e| EncodeError::WebPEncoderCreation(e.to_string()))?;

            encoder
                .encode_advanced(
                    &WebPConfig::new_with_preset(WebPPreset::WEBP_PRESET_PHOTO, 85.0).unwrap(),
                )
                .map_err(|e| EncodeError::WebPEncode(format!("{:?}", e)))
                .map(|encoded| encoded.to_vec())
        }
    }
}

#[derive(Error, Debug)]
pub enum UploadError {
    #[error("Failed to upload to bucket: {0}")]
    BucketUpload(#[from] Box<dyn std::error::Error + Send + Sync>),
}

async fn upload_image(
    ctx: &Context,
    media: &Media,
    buffer: Vec<u8>,
    spec: &ImageSpec,
    content_type: &ImageContentType,
) -> Result<(), UploadError> {
    let key = generate_resized_media_key(GenerateResizedMediaKeyParams {
        media_id: media.id,
        user_id: media.user_id,
        content_type: content_type.clone(),
        image_spec: spec.clone(),
    });

    ctx.bucket_client
        .put_object(
            &key,
            buffer.into(),
            ObjectParams {
                content_type: Some(content_type.to_string()),
            },
        )
        .await
        .map_err(|e| UploadError::BucketUpload(Box::new(e)))?;

    info!("Uploaded resized image: {}", key);
    Ok(())
}

#[derive(Error, Debug)]
pub enum MediaJobError {
    #[error("Database error: {0}")]
    Database(anyhow::Error),
    #[error("Failed to get media from repository: {0}")]
    MediaNotFound(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Media object not found in bucket: {0}")]
    BucketObjectNotFound(String),
    #[error("Failed to read image: {0}")]
    ImageRead(#[from] image::error::ImageError),
    #[error("Failed to encode image: {0}")]
    Encode(#[from] EncodeError),
    #[error("Failed to upload image: {0}")]
    Upload(#[from] UploadError),
    #[error("Task join error: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),
    #[error("RecvError: {0}")]
    RecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Location infer failed")]
    LocationInferFailed(anyhow::Error),
    #[error("Semaphore error: {0}")]
    Semaphore(#[from] tokio::sync::AcquireError),
    #[error("Failed to infer media type: {0}")]
    InferFailed(String),
    #[error("Failed to load HEIF image: {0}")]
    HeifError(#[from] libheif_rs::HeifError),
}

pub async fn handle_media_job(job: MediaJob, ctx: Context) -> Result<(), MediaJobError> {
    info!("Handling job: {:?}", job);

    let Context {
        bucket_client,
        repos:
            Repos {
                media_repo,
                ride_repo,
                ride_points_repo,
                ..
            },
        ..
    } = ctx.clone();

    match job {
        MediaJob::Process(media_id) => {
            let media = media_repo
                .get(media_id)
                .await
                .map_err(MediaJobError::Database)?;

            let _permit = ctx.image_processing_semaphore.acquire().await?;

            let bytes = bucket_client
                .get_object(&media.path)
                .await
                .map_err(|e| MediaJobError::MediaNotFound(Box::new(e)))?
                .ok_or_else(|| MediaJobError::BucketObjectNotFound(media.path.clone()))?;

            let size_kb = bytes.len() as f64 / 1024.0;
            let kind =
                infer::get(&bytes).ok_or_else(|| MediaJobError::InferFailed(media.path.clone()))?;

            info!("Media file size: {:.2} KB", size_kb);
            info!("Media kind: {:?}", &kind.mime_type());

            let img = match kind.mime_type() {
                "image/heif" => {
                    let lib_heif = LibHeif::new();
                    let ctx = HeifContext::read_from_bytes(&bytes)?;
                    let handle = ctx.primary_image_handle()?;
                    let heif_image =
                        lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

                    let img = RgbImage::from_raw(
                        heif_image.width(),
                        heif_image.height(),
                        heif_image.planes().interleaved.unwrap().data.to_vec(),
                    )
                    .unwrap();

                    DynamicImage::from(img)
                }
                _ => ImageReader::new(Cursor::new(&bytes))
                    .with_guessed_format()?
                    .decode()?,
            };

            for image_spec in IMAGE_SPECS.iter() {
                let img_clone = img.clone();

                let resized = rayon_spawn_blocking(move || resize(&img_clone, &image_spec)).await;

                for content_type in [ImageContentType::Jpeg, ImageContentType::Webp] {
                    let resized_clone = resized.clone();
                    let content_type_clone = content_type.clone();

                    let buffer =
                        rayon_spawn_blocking(move || encode(&resized_clone, &content_type_clone))
                            .await?;

                    upload_image(&ctx, &media, buffer, image_spec, &content_type).await?;
                }
            }

            Ok(())
        }
        MediaJob::InferLocation(media_id) => {
            let media = media_repo
                .get(media_id)
                .await
                .map_err(MediaJobError::Database)?;

            let inferrer = MediaGeoInferrer::new(
                media_repo.clone(),
                ride_repo.clone(),
                ride_points_repo.clone(),
            );

            inferrer
                .infer_ride_and_point_and_save(&media)
                .await
                .map_err(MediaJobError::LocationInferFailed)?;

            Ok(())
        }
    }
}
