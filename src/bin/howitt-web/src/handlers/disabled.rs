use async_graphql::ErrorExtensions;
use axum::{http::StatusCode, Json};

pub const MESSAGE: &str = "Background jobs are disabled on this deployment";

pub fn graphql_error() -> async_graphql::Error {
    async_graphql::Error::new(MESSAGE)
        .extend_with(|_, extensions| extensions.set("code", "BACKGROUND_JOBS_DISABLED"))
}

/// No extractors that read bodies, database state, or OAuth tokens: reject before
/// any upload, token exchange, database write, or job enqueue can happen.
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
        body::{to_bytes, Body},
        http::Request,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn job_routes_reject_without_state_or_valid_body() {
        for (method, path) in [
            ("POST", "/upload/media"),
            ("POST", "/webhooks/rwgps"),
            ("GET", "/auth/rwgps/callback"),
        ] {
            let response = crate::disabled_routes()
                .oneshot(
                    Request::builder()
                        .method(method)
                        .uri(path)
                        .header("origin", "https://howittplains.net")
                        .body(Body::from("invalid body"))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), 503);
            assert_eq!(response.headers()["access-control-allow-origin"], "*");
            let body = to_bytes(response.into_body(), 4096).await.unwrap();
            let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(json["code"], "BACKGROUND_JOBS_DISABLED");
        }
    }

    #[tokio::test]
    async fn sync_mutation_rejects_before_accessing_repositories() {
        let schema = async_graphql::Schema::build(
            crate::graphql::schema::query::Query,
            crate::graphql::schema::mutation::Mutation,
            async_graphql::EmptySubscription,
        )
        .finish();
        let response = schema
            .execute("mutation { initiateRwgpsHistorySync { id } }")
            .await;
        assert_eq!(response.errors.len(), 1);
        assert_eq!(response.errors[0].message, super::MESSAGE);
        assert_eq!(
            response.errors[0].extensions.as_ref().unwrap().get("code"),
            Some(&async_graphql::Value::from("BACKGROUND_JOBS_DISABLED"))
        );
    }
}
