use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
};
use chrono::Utc;
use howitt::{
    models::{
        media::{Media, MediaId, MediaRelationId},
        user::UserId,
    },
    repos::Repos,
    services::{
        media::{MediaGeoInferrer, keys::cloudflare_image_path},
        user::auth::Login,
    },
};

use crate::app_state::AppState;

// Cloudflare hosted Images limit. No pixel decoding or transformations here.
pub const MAX_FILE_BYTES: usize = 10_000_000;
type UploadError = (StatusCode, &'static str);
const BAD_UPLOAD: UploadError = (StatusCode::BAD_REQUEST, "Invalid media upload");
const INTERNAL: UploadError = (StatusCode::INTERNAL_SERVER_ERROR, "Media upload failed");

#[derive(Clone)]
pub struct ImagesConfig {
    endpoint: String,
    delivery_hash: String,
    token: String,
}

impl ImagesConfig {
    pub fn new(account: String, delivery_hash: String, token: String) -> anyhow::Result<Self> {
        anyhow::ensure!(
            account.len() == 32 && account.bytes().all(|b| b.is_ascii_hexdigit()),
            "Invalid Images account ID"
        );
        anyhow::ensure!(
            cloudflare_image_path(&delivery_hash, "validation").is_some(),
            "Invalid Images delivery hash"
        );
        anyhow::ensure!(!token.trim().is_empty(), "Missing Images token");
        Ok(Self {
            endpoint: format!("https://api.cloudflare.com/client/v4/accounts/{account}/images/v1"),
            delivery_hash,
            token,
        })
    }

    async fn upload(
        &self,
        media: &Media,
        bytes: Vec<u8>,
        name: String,
        mime: &str,
    ) -> anyhow::Result<()> {
        let id = media.id.as_uuid().to_string();
        let form = reqwest::multipart::Form::new()
            .text("id", id.clone())
            .text("creator", media.user_id.to_string())
            .text("requireSignedURLs", "false")
            .text(
                "metadata",
                serde_json::json!({"media_id": media.id.to_string()}).to_string(),
            )
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes)
                    .file_name(name)
                    .mime_str(mime)?,
            );
        let response = reqwest::Client::new()
            .post(&self.endpoint)
            .bearer_auth(&self.token)
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;
        let response: serde_json::Value = response.json().await?;
        anyhow::ensure!(
            response["success"] == true && response["result"]["id"] == id,
            "Images did not confirm upload"
        );
        Ok(())
    }
}

struct Upload {
    bytes: Vec<u8>,
    name: String,
    mime: String,
    relations: Vec<MediaRelationId>,
}

async fn read_upload(mut multipart: Multipart) -> Result<Upload, UploadError> {
    let mut file = None;
    let mut name = None;
    let mut relations = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (e.status(), "Invalid multipart upload"))?
    {
        match field.name() {
            Some("file") if file.is_none() => {
                let mime = field.content_type().unwrap_or("").to_string();
                // Do not serve SVG or accept arbitrary non-image uploads. Images validates contents.
                if !matches!(
                    mime.as_str(),
                    "image/jpeg"
                        | "image/png"
                        | "image/gif"
                        | "image/webp"
                        | "image/heic"
                        | "image/heif"
                ) {
                    return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported image type"));
                }
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| (e.status(), "Invalid image body"))?;
                if bytes.len() > MAX_FILE_BYTES {
                    return Err((StatusCode::PAYLOAD_TOO_LARGE, "Image exceeds 10 MB"));
                }
                if bytes.is_empty() {
                    return Err(BAD_UPLOAD);
                }
                file = Some((bytes.to_vec(), mime));
            }
            Some("name") if name.is_none() => {
                let value = field.text().await.map_err(|_| BAD_UPLOAD)?;
                if value.is_empty() || value.len() > 255 || value.chars().any(char::is_control) {
                    return Err(BAD_UPLOAD);
                }
                name = Some(value);
            }
            Some("relation_ids") if relations.is_none() => {
                let text = field.text().await.map_err(|_| BAD_UPLOAD)?;
                let ids: Vec<MediaRelationId> =
                    serde_json::from_str(&text).map_err(|_| BAD_UPLOAD)?;
                if ids.len() > 20 {
                    return Err(BAD_UPLOAD);
                }
                relations = Some(ids);
            }
            _ => return Err(BAD_UPLOAD),
        }
    }
    let (bytes, mime) = file.ok_or(BAD_UPLOAD)?;
    Ok(Upload {
        bytes,
        mime,
        name: name.ok_or(BAD_UPLOAD)?,
        relations: relations.ok_or(BAD_UPLOAD)?,
    })
}

async fn authorize_relations(
    repos: &Repos,
    relations: &[MediaRelationId],
    user_id: UserId,
) -> Result<(), UploadError> {
    for relation in relations {
        let owner = match relation {
            MediaRelationId::Ride(id) => repos.ride_repo.get(*id).await.map(|v| v.user_id),
            MediaRelationId::Route(id) => repos.route_repo.get(*id).await.map(|v| v.user_id),
            MediaRelationId::Trip(id) => repos.trip_repo.get(*id).await.map(|v| v.user_id),
            MediaRelationId::PointOfInterest(id) => repos
                .point_of_interest_repo
                .get(*id)
                .await
                .map(|v| v.user_id),
        }
        .map_err(|_| (StatusCode::FORBIDDEN, "Media relation is unavailable"))?;
        if owner != user_id {
            return Err((StatusCode::FORBIDDEN, "Media relation is unavailable"));
        }
    }
    Ok(())
}

pub async fn handler(
    State(state): State<AppState>,
    login: Login,
    multipart: Multipart,
) -> Result<(StatusCode, Json<serde_json::Value>), UploadError> {
    let future = upload(state, login, multipart);
    #[cfg(target_arch = "wasm32")]
    return worker::send::SendFuture::new(future).await;
    #[cfg(not(target_arch = "wasm32"))]
    future.await
}

async fn upload(
    state: AppState,
    login: Login,
    multipart: Multipart,
) -> Result<(StatusCode, Json<serde_json::Value>), UploadError> {
    let images = state.images.ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Images uploads are not configured",
    ))?;
    let upload = read_upload(multipart).await?;
    let user_id = login.session.user_id;
    authorize_relations(&state.repos, &upload.relations, user_id).await?;
    let exif = exif::parse_exif(&upload.bytes);
    let id = MediaId::new();
    let mut media = Media {
        id,
        user_id,
        created_at: Utc::now(),
        path: cloudflare_image_path(&images.delivery_hash, &id.as_uuid().to_string())
            .ok_or(INTERNAL)?,
        relation_ids: upload.relations,
        point: exif.point.filter(|p| {
            p.x().is_finite()
                && p.y().is_finite()
                && (-180.0..=180.0).contains(&p.x())
                && (-90.0..=90.0).contains(&p.y())
        }),
        captured_at: exif.captured_at,
    };
    // Resolve metadata before creating any provider object. Preserve real EXIF GPS.
    // This uses the same domain service as the queue consumer without a DB/queue dual write.
    if media.point.is_none() && media.captured_at.is_some() {
        let inferrer = MediaGeoInferrer::new(
            state.repos.media_repo.clone(),
            state.repos.ride_repo.clone(),
            state.repos.ride_points_repo.clone(),
        );
        if let Some(location) = inferrer
            .infer_ride_and_point(&media)
            .await
            .map_err(|_| INTERNAL)?
        {
            media.point = Some(location.point);
        }
    }
    // A failed/ambiguous upload or DB save is never reported as success. Retain
    // provider originals on DB failure; the media_id metadata permits reconciliation.
    images
        .upload(&media, upload.bytes, upload.name, &upload.mime)
        .await
        .map_err(|_| {
            #[cfg(target_arch = "wasm32")]
            worker::console_error!(
                "Images upload unconfirmed media_id={}; reconcile before retry",
                id
            );
            #[cfg(not(target_arch = "wasm32"))]
            tracing::error!(media_id = %id, "Images upload unconfirmed; reconcile before retry");
            (StatusCode::BAD_GATEWAY, "Images upload was not confirmed")
        })?;
    state.repos.media_repo.put(media).await.map_err(|_| {
        #[cfg(target_arch = "wasm32")]
        worker::console_error!("Images database save failed media_id={}; original retained", id);
        #[cfg(not(target_arch = "wasm32"))]
        tracing::error!(media_id = %id, "Images upload confirmed but database save failed; original retained");
        INTERNAL
    })?;
    Ok((StatusCode::CREATED, Json(serde_json::json!({"id": id}))))
}
