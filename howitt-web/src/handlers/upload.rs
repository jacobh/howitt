use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use howitt::{
    models::media::{Media, MediaId},
    repos::Repo,
    services::user::auth::Login,
};
use howitt_client_types::BucketClient;
use serde_json::json;

use crate::app_state::AppState;

pub async fn upload_media_handler(
    State(state): State<AppState>,
    login: Login,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Get the first file from the multipart request
    let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Failed to get field: {}", e)})),
        )
    })?
    else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No file provided"})),
        ));
    };

    // Get the file bytes
    let bytes = field.bytes().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Failed to read file: {}", e)})),
        )
    })?;

    // Generate unique ID and S3 key
    let media_id = MediaId::new();
    let key = format!("media/{}", media_id);

    // Upload to S3
    state
        .bucket_client
        .put_object(&key, bytes)
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
