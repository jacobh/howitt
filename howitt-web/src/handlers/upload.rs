use apalis::prelude::*;
use axum::{extract::State, http::StatusCode, Json};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use howitt::{
    jobs::{
        media::{MediaJob, ProcessMedia},
        Job,
    },
    models::{
        media::{Media, MediaId, MediaRelationId},
        user::UserId,
    },
    repos::Repo,
    services::user::auth::Login,
};
use howitt_client_types::{BucketClient, ObjectParams};
use sanitize_filename::sanitize;
use serde_json::json;
use tempfile::NamedTempFile;

use crate::app_state::AppState;

#[derive(TryFromMultipart)]
pub struct UploadMediaRequest {
    #[form_data(limit = "unlimited")]
    pub file: FieldData<NamedTempFile>,
    pub name: String,
    pub relation_ids: Option<String>, // JSON array of relation IDs
}

fn parse_relation_ids(
    relation_ids: Option<String>,
) -> Result<Vec<MediaRelationId>, serde_json::Error> {
    relation_ids
        .map(|ids| serde_json::from_str(&ids))
        .unwrap_or(Ok(vec![]))
}

pub struct GenerateMediaKeyParams {
    media_id: MediaId,
    user_id: UserId,
    name: String,
}

pub fn generate_media_key(
    GenerateMediaKeyParams {
        media_id,
        user_id,
        name,
    }: GenerateMediaKeyParams,
) -> String {
    format!(
        "originals/user/{}/media/{}/{}",
        user_id.as_uuid(),
        media_id.as_uuid(),
        sanitize(&name)
    )
}

pub async fn upload_media_handler(
    State(AppState {
        bucket_client,
        media_repo,
        job_storage,
        ..
    }): State<AppState>,
    login: Login,
    TypedMultipart(upload): TypedMultipart<UploadMediaRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Generate unique ID and S3 key
    let media_id = MediaId::new();
    let key = generate_media_key(GenerateMediaKeyParams {
        media_id,
        user_id: login.session.user_id,
        name: upload.name,
    });

    // Get the file contents
    let file = upload.file.contents;
    let bytes = std::fs::read(file.path()).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to read temp file: {}", e)})),
        )
    })?;

    let kind = infer::get(&bytes).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Could not determine file type"})),
        )
    })?;

    let params = ObjectParams {
        content_type: Some(kind.mime_type().to_string()),
    };

    // Upload to S3
    bucket_client
        .put_object(&key, bytes.into(), params)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to upload to S3: {}", e)})),
            )
        })?;

    let relation_ids = parse_relation_ids(upload.relation_ids).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to parse relation ids: {}", e)})),
        )
    })?;

    // Create media record
    let media = Media {
        id: media_id.clone(),
        created_at: chrono::Utc::now(),
        user_id: login.session.user_id,
        path: key,
        relation_ids,
    };

    // Save to database
    media_repo.put(media).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to save to database: {}", e)})),
        )
    })?;

    job_storage
        .lock()
        .await
        .push(Job::from(MediaJob::from(ProcessMedia {
            media_id: media_id.clone(),
        })))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to enqueue job: {}", e)})),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "media_id": media_id.to_string()
        })),
    ))
}

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_media_key() {
        let params = GenerateMediaKeyParams {
            media_id: MediaId::from(
                uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            ),
            user_id: UserId::from(
                uuid::Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            ),
            name: "test-image.jpg".to_string(),
        };

        let key = generate_media_key(params);
        assert_eq!(
            key,
            "originals/user/123e4567-e89b-12d3-a456-426614174000/media/550e8400-e29b-41d4-a716-446655440000/test-image.jpg"
        );
    }

    #[test]
    fn test_generate_media_key_with_special_chars() {
        let params = GenerateMediaKeyParams {
            media_id: MediaId::from(
                uuid::Uuid::parse_str("7f9c24e5-2c44-4a8e-95d3-a515bf484018").unwrap(),
            ),
            user_id: UserId::from(
                uuid::Uuid::parse_str("9d25a949-c374-4f22-9ca8-1c17d4982384").unwrap(),
            ),
            name: "test image/with spaces!@#$.jpg".to_string(),
        };

        let key = generate_media_key(params);
        assert_eq!(
            key,
            "originals/user/9d25a949-c374-4f22-9ca8-1c17d4982384/media/7f9c24e5-2c44-4a8e-95d3-a515bf484018/test imagewith spaces!@#$.jpg"
        );
    }
}
