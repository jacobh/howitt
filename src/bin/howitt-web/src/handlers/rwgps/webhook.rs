use axum::{
    extract::{FromRequestParts, State},
    http::StatusCode,
};
use howitt::jobs::{rwgps::RwgpsJob, Job};
use http::request::Parts;
use ring::hmac;
use rwgps_types::webhook::RwgpsWebhookPayload;

use crate::app_state::AppState;

pub struct RwgpsSignature(Vec<u8>);

impl RwgpsSignature {
    pub fn verify(&self, secret: &str, body: &str) -> Result<(), ring::error::Unspecified> {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
        hmac::verify(&key, body.as_bytes(), &self.0)
    }
}

impl<S> FromRequestParts<S> for RwgpsSignature
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the signature from headers
        let signature_hex = match parts.headers.get("x-rwgps-signature") {
            Some(sig) => match sig.to_str() {
                Ok(s) => s.to_string(),
                Err(e) => {
                    tracing::error!("Invalid signature header: {}", e);
                    return Err((StatusCode::BAD_REQUEST, "Invalid signature header"));
                }
            },
            None => {
                tracing::error!("Missing x-rwgps-signature header");
                return Err((StatusCode::BAD_REQUEST, "Missing x-rwgps-signature header"));
            }
        };

        // Parse hex string into bytes
        let signature_bytes = match hex::decode(&signature_hex) {
            Ok(bytes) => bytes,
            Err(e) => {
                tracing::error!("Invalid hex signature: {}", e);
                return Err((StatusCode::BAD_REQUEST, "Invalid signature format"));
            }
        };

        Ok(RwgpsSignature(signature_bytes))
    }
}

pub async fn rwgps_webhook_handler(
    State(app_state): State<AppState>,
    signature: RwgpsSignature,
    body: String,
) -> StatusCode {
    let AppState { job_storage, .. } = app_state;

    if let Err(e) = signature.verify(&app_state.rwgps.client_secret, &body) {
        tracing::error!("Invalid signature: {}", e);
        return StatusCode::UNAUTHORIZED;
    }

    // Parse and log the webhook payload
    let payload = match serde_json::from_str::<RwgpsWebhookPayload>(&body) {
        Ok(payload) => {
            tracing::info!("Received RWGPS webhook: {:?}", payload);
            payload
        }
        Err(e) => {
            tracing::error!("Failed to parse webhook payload: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    for notification in payload.notifications.into_iter() {
        let res = job_storage
            .push(Job::Rwgps(RwgpsJob::Webhook(notification)))
            .await;

        if let Err(e) = res {
            tracing::error!("Failed to push job: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    StatusCode::OK
}
