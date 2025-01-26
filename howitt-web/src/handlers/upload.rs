use axum::{extract::State, http::StatusCode, Json};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use howitt::{
    models::media::{Media, MediaId},
    repos::Repo,
    services::user::auth::Login,
};
use howitt_client_types::BucketClient;
use serde_json::json;
use tempfile::NamedTempFile;

use crate::app_state::AppState;

#[derive(TryFromMultipart)]
pub struct UploadMediaRequest {
    #[form_data(limit = "unlimited")]
    pub file: FieldData<NamedTempFile>,
}

pub async fn upload_media_handler(
    State(state): State<AppState>,
    login: Login,
    TypedMultipart(upload): TypedMultipart<UploadMediaRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Generate unique ID and S3 key
    let media_id = MediaId::new();
    let key = format!("media/{}", media_id);

    // Get the file contents
    let file = upload.file.contents;
    let bytes = std::fs::read(file.path()).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to read temp file: {}", e)})),
        )
    })?;

    // Upload to S3
    state
        .bucket_client
        .put_object(&key, bytes.into())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to upload to S3: {}", e)})),
            )
        })?;

    // Create media record
    let media = Media {
        id: MediaId::from(media_id),
        created_at: chrono::Utc::now(),
        user_id: login.session.user_id,
        path: key,
        relation_ids: vec![],
    };

    // Save to database
    state.media_repo.put(media).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to save to database: {}", e)})),
        )
    })?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "media_id": media_id.to_string()
        })),
    ))
}
