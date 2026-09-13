use axum::{
    extract::{FromRequestParts, State},
    http::StatusCode,
};
use howitt::jobs::{Job, rwgps::RwgpsJob};
use http::request::Parts;
use ring::hmac;
use rwgps_types::webhook::RwgpsWebhookPayload;

use crate::app_state::RwgpsWebhookState;

pub(crate) struct RwgpsSignature(Vec<u8>);

impl RwgpsSignature {
    fn verify(&self, secret: &str, body: &[u8]) -> Result<(), ring::error::Unspecified> {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
        hmac::verify(&key, body, &self.0)
    }
}

impl<S> FromRequestParts<S> for RwgpsSignature
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let signature = parts
            .headers
            .get("x-rwgps-signature")
            .ok_or((StatusCode::BAD_REQUEST, "Missing x-rwgps-signature header"))?
            .to_str()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid signature header"))?;
        let bytes = hex::decode(signature)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid signature format"))?;
        Ok(Self(bytes))
    }
}

pub async fn handler(
    State(state): State<RwgpsWebhookState>,
    signature: RwgpsSignature,
    body: axum::body::Bytes,
) -> StatusCode {
    if signature.verify(&state.client_secret, &body).is_err() {
        return StatusCode::UNAUTHORIZED;
    }

    let payload = match serde_json::from_slice::<RwgpsWebhookPayload>(&body) {
        Ok(payload) => payload,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    let jobs = payload
        .notifications
        .into_iter()
        .map(|notification| Job::Rwgps(RwgpsJob::Webhook(notification)))
        .collect();

    match state.jobs.publish(jobs).await {
        Ok(()) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jobs::JobQueue;
    use axum::{
        Router,
        body::{Body, to_bytes},
        http::Request,
        routing::post,
    };
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;

    #[derive(Default)]
    struct RecordingQueue(Mutex<Vec<Job>>);

    struct FailingQueue;

    #[async_trait::async_trait]
    impl JobQueue for RecordingQueue {
        async fn publish(&self, jobs: Vec<Job>) -> anyhow::Result<()> {
            self.0.lock().unwrap().extend(jobs);
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl JobQueue for FailingQueue {
        async fn publish(&self, _: Vec<Job>) -> anyhow::Result<()> {
            anyhow::bail!("synthetic queue failure")
        }
    }

    fn signed_request(body: &str, secret: &str) -> Request<Body> {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
        let signature = hex::encode(hmac::sign(&key, body.as_bytes()).as_ref());
        Request::builder()
            .method("POST")
            .uri("/webhooks/rwgps")
            .header("x-rwgps-signature", signature)
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn verifies_raw_body_and_publishes_every_notification() {
        let queue = Arc::new(RecordingQueue::default());
        let app = Router::new()
            .route("/webhooks/rwgps", post(handler))
            .with_state(RwgpsWebhookState {
                client_secret: "test-secret".into(),
                jobs: queue.clone(),
            });
        let body = r#"{"notifications":[{"user_id":7,"item_type":"route","item_id":11,"item_user_id":7,"item_url":"https://ridewithgps.com/routes/11.json","action":"updated","collection":null},{"user_id":7,"item_type":"trip","item_id":12,"item_user_id":7,"item_url":"https://ridewithgps.com/trips/12.json","action":"created","collection":null}]}"#;

        let response = app
            .clone()
            .oneshot(signed_request(body, "test-secret"))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(queue.0.lock().unwrap().len(), 2);

        let response = app
            .oneshot(signed_request(body, "wrong-secret"))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(
            to_bytes(response.into_body(), 1024)
                .await
                .unwrap()
                .is_empty()
        );
        assert_eq!(queue.0.lock().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn rejects_malformed_payload_and_reports_publication_failure() {
        let malformed = Router::new()
            .route("/webhooks/rwgps", post(handler))
            .with_state(RwgpsWebhookState {
                client_secret: "test-secret".into(),
                jobs: Arc::new(RecordingQueue::default()),
            })
            .oneshot(signed_request("not-json", "test-secret"))
            .await
            .unwrap();
        assert_eq!(malformed.status(), StatusCode::BAD_REQUEST);

        let body = r#"{"notifications":[{"user_id":7,"item_type":"route","item_id":11,"item_user_id":7,"item_url":"https://ridewithgps.com/routes/11.json","action":"updated","collection":null}]}"#;
        let failed = Router::new()
            .route("/webhooks/rwgps", post(handler))
            .with_state(RwgpsWebhookState {
                client_secret: "test-secret".into(),
                jobs: Arc::new(FailingQueue),
            })
            .oneshot(signed_request(body, "test-secret"))
            .await
            .unwrap();
        assert_eq!(failed.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
