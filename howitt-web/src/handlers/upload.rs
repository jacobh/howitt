use apalis::prelude::*;
use axum::{body::Bytes, extract::State, http::StatusCode, Json};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use howitt::{
    jobs::{
        media::{MediaJob, ProcessMedia},
        Job,
    },
    models::media::{Media, MediaId, MediaRelationId},
    repos::Repo,
    services::{
        media::{generate_media_key, GenerateMediaKeyParams},
        user::auth::Login,
    },
};
use howitt_client_types::{BucketClient, ObjectParams};
use serde_json::json;

use crate::app_state::AppState;

#[derive(TryFromMultipart)]
pub struct UploadMediaRequest {
    #[form_data(limit = "unlimited")]
    pub file: FieldData<Bytes>,
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
    let bytes = upload.file.contents;

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
        point: None,
        captured_at: None,
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
