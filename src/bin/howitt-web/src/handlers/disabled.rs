use axum::{Json, http::StatusCode};

pub const MESSAGE: &str = "Background jobs are disabled on this deployment";

/// Reject media before reading an upload body or changing storage/database state.
pub async fn handler() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": MESSAGE,
            "code": "BACKGROUND_JOBS_DISABLED",
        })),
    )
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::Request,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn media_upload_rejects_without_state_or_valid_body() {
        let response = crate::disabled_routes()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/upload/media")
                    .header("origin", "https://howittplains.net")
                    .body(Body::from("invalid body"))
                    .unwrap(),
            )
            .await;
        let response = response.unwrap();
        assert_eq!(response.status(), 503);
        assert_eq!(response.headers()["access-control-allow-origin"], "*");
        let body = to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["code"], "BACKGROUND_JOBS_DISABLED");
    }
}
