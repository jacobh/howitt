use axum::{extract::State, response::IntoResponse, Json};
use chrono::Utc;
use howitt::{models::user::UserRwgpsConnection, repos::Repo};
use http::StatusCode;
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};
use oauth2::{
    Client, EndpointNotSet, ExtraTokenFields, StandardRevocableToken, StandardTokenResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct RwgpsCallbackParams {
    code: String,
    state: String, // jwt
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RideWithGpsExtraTokenFields {
    created_at: Option<i64>,
    user_id: Option<i64>,
}

impl ExtraTokenFields for RideWithGpsExtraTokenFields {}

type RwgpsTokenResponse =
    StandardTokenResponse<RideWithGpsExtraTokenFields, oauth2::basic::BasicTokenType>;

type RwgpsClient<
    HasAuthUrl = EndpointNotSet,
    HasDeviceAuthUrl = EndpointNotSet,
    HasIntrospectionUrl = EndpointNotSet,
    HasRevocationUrl = EndpointNotSet,
    HasTokenUrl = EndpointNotSet,
> = Client<
    BasicErrorResponse,
    RwgpsTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    HasAuthUrl,
    HasDeviceAuthUrl,
    HasIntrospectionUrl,
    HasRevocationUrl,
    HasTokenUrl,
>;

pub async fn rwgps_callback_handler(
    State(app_state): State<AppState>,
    Json(params): Json<RwgpsCallbackParams>,
) -> impl IntoResponse {
    let login = match app_state.user_auth_service.verify(&params.state).await {
        Ok(login) => login,
        Err(e) => {
            tracing::error!("Failed to verify auth state: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid authentication state"
                })),
            );
        }
    };

    let client = RwgpsClient::new(ClientId::new(app_state.rwgps.client_id))
        .set_client_secret(ClientSecret::new(app_state.rwgps.client_secret))
        .set_auth_uri(AuthUrl::new("https://ridewithgps.com/oauth/authorize".to_string()).unwrap())
        .set_token_uri(
            TokenUrl::new("https://ridewithgps.com/oauth/token.json".to_string()).unwrap(),
        )
        .set_redirect_uri(RedirectUrl::new(app_state.rwgps.redirect_uri).unwrap());

    let http_client = oauth2::reqwest::Client::new();

    // Exchange the authorization code for an access token
    let token_result = client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(&http_client)
        .await;

    let token = match token_result {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("Failed to exchange RWGPS auth code: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Failed to exchange authorization code"
                })),
            );
        }
    };

    // Extract the RWGPS user ID from the token response
    //  this `EmptyExtraTokenFields` is a placeholder and should be replaced with the actual extra fields type if available
    let extra_params = token.extra_fields();
    let rwgps_user_id = match extra_params.user_id {
        Some(user_id) => user_id as i32,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Missing RWGPS user ID in token response"
                })),
            );
        }
    };

    // Create RWGPS connection record
    let rwgps_connection = UserRwgpsConnection {
        id: Uuid::new_v4(),
        user_id: login.session.user_id,
        rwgps_user_id,
        access_token: token.access_token().secret().to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Get existing user record
    let mut user = match app_state.user_repo.get(login.session.user_id).await {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch user record"
                })),
            );
        }
    };

    // Update user with RWGPS connection
    user.rwgps_connection = Some(rwgps_connection);

    // Save updated user
    if let Err(e) = app_state.user_repo.put(user).await {
        tracing::error!("Failed to save user with RWGPS connection: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to save RWGPS connection"
            })),
        );
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Successfully linked RWGPS account"
        })),
    )
}
