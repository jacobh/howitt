use howitt::models::media::{ImageContentType, ImageDimensions, ImageSpec, Media, IMAGE_SPECS};
use howitt::services::media::keys::{generate_resized_media_key, GenerateResizedMediaKeyParams};
use howitt_postgresql::PostgresRepoError;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageReader};
use libwebp_sys::WebPPreset;
use std::io::Cursor;
use thiserror::Error;
use webp::WebPConfig;

use apalis::prelude::*;
use tracing::info;

use howitt::{
    jobs::media::{MediaJob, ProcessMedia},
    repos::Repo,
};
use howitt_client_types::{BucketClient, ObjectParams};

use crate::context::Context;

fn resize(img: &DynamicImage, spec: &ImageSpec) -> DynamicImage {
    match spec {
        ImageSpec::Fill(dimensions) => match dimensions {
            ImageDimensions::Square(size) => {
                let (width, height) = img.dimensions();
                let (x, y, crop_size) = if width > height {
                    let x = (width - height) / 2;
                    (x, 0, height)
                } else {
                    let y = (height - width) / 2;
                    (0, y, width)
                };

                let cropped = img.crop_imm(x, y, crop_size, crop_size);
                cropped.resize_to_fill(*size as u32, *size as u32, FilterType::Lanczos3)
            }
            ImageDimensions::Rectangle { .. } => {
                unimplemented!("Rectangle fill dimensions not yet supported")
            }
        },
        ImageSpec::Fit(dimensions) => match dimensions {
            ImageDimensions::Square(size) => {
                let target = *size as u32;
                img.resize_to_fill(target, target, FilterType::Lanczos3)
            }
            ImageDimensions::Rectangle { .. } => {
                unimplemented!("Rectangle fit dimensions not yet supported")
            }
        },
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
    Database(#[from] PostgresRepoError),
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
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub async fn handle_media_job(job: MediaJob, ctx: Data<Context>) -> Result<(), MediaJobError> {
    info!("Handling job: {:?}", job);

    match job {
        MediaJob::Process(ProcessMedia { media_id }) => {
            let media = ctx.media_repo.get(media_id).await?;

            let bytes = ctx
                .bucket_client
                .get_object(&media.path)
                .await
                .map_err(|e| MediaJobError::MediaNotFound(Box::new(e)))?
                .ok_or_else(|| MediaJobError::BucketObjectNotFound(media.path.clone()))?;

            let size_kb = bytes.len() as f64 / 1024.0;
            info!("Media file size: {:.2} KB", size_kb);

            let img = ImageReader::new(Cursor::new(&bytes))
                .with_guessed_format()?
                .decode()?;

            for image_spec in IMAGE_SPECS.iter() {
                let img_clone = img.clone();
                let resized =
                    tokio::task::spawn_blocking(move || resize(&img_clone, image_spec)).await?;

                for content_type in [ImageContentType::Jpeg, ImageContentType::Webp] {
                    let resized_clone = resized.clone();
                    let content_type_clone = content_type.clone();

                    let buffer = tokio::task::spawn_blocking(move || {
                        encode(&resized_clone, &content_type_clone)
                    })
                    .await??;

                    upload_image(&ctx, &media, buffer, image_spec, &content_type).await?;
                }
            }

            Ok(())
        }
    }
}
