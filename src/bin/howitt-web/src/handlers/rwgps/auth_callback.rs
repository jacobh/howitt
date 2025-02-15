use axum::extract::Query;
use axum::response::{Redirect, Response};
use axum::{extract::State, response::IntoResponse, Json};
use chrono::Utc;
use howitt::jobs::rwgps::RwgpsJob;
use howitt::jobs::Job;
use howitt::models::user::UserRwgpsConnection;
use howitt::repos::Repos;
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
    State(AppState {
        user_auth_service,
        repos: Repos { user_repo, .. },
        rwgps,
        job_storage,
        ..
    }): State<AppState>,
    Query(params): Query<RwgpsCallbackParams>,
) -> Response {
    tracing::info!("Handling RWGPS callback with state token");

    let login = match user_auth_service.verify(&params.state).await {
        Ok(login) => {
            tracing::info!(user_id = %login.session.user_id, "Successfully verified auth state");
            login
        }
        Err(e) => {
            tracing::error!("Failed to verify auth state: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid authentication state"
                })),
            )
                .into_response();
        }
    };

    tracing::debug!("Creating RWGPS OAuth client");
    let client = RwgpsClient::new(ClientId::new(rwgps.client_id))
        .set_client_secret(ClientSecret::new(rwgps.client_secret))
        .set_auth_uri(AuthUrl::new("https://ridewithgps.com/oauth/authorize".to_string()).unwrap())
        .set_token_uri(
            TokenUrl::new("https://ridewithgps.com/oauth/token.json".to_string()).unwrap(),
        )
        .set_redirect_uri(RedirectUrl::new(rwgps.redirect_uri).unwrap());

    let http_client = oauth2::reqwest::Client::new();

    tracing::info!("Exchanging authorization code for access token");
    // Exchange the authorization code for an access token
    let token_result = client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(&http_client)
        .await;

    let token = match token_result {
        Ok(token) => {
            tracing::info!("Successfully exchanged authorization code for access token");
            token
        }
        Err(e) => {
            tracing::error!("Failed to exchange RWGPS auth code: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Failed to exchange authorization code"
                })),
            )
                .into_response();
        }
    };

    tracing::debug!("Extracting RWGPS user ID from token response");
    let extra_params = token.extra_fields();
    let rwgps_user_id = match extra_params.user_id {
        Some(user_id) => {
            tracing::info!(
                rwgps_user_id = user_id,
                "Found RWGPS user ID in token response"
            );
            user_id as i32
        }
        None => {
            tracing::error!("Token response missing RWGPS user ID");
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                        "error": "Missing RWGPS user ID in token response"
                })),
            )
                .into_response();
        }
    };

    tracing::debug!("Creating RWGPS connection record");
    let rwgps_connection = UserRwgpsConnection {
        id: Uuid::new_v4(),
        user_id: login.session.user_id,
        rwgps_user_id,
        access_token: token.access_token().secret().to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    tracing::info!(user_id = %login.session.user_id, "Fetching user record");
    let mut user = match user_repo.get(login.session.user_id).await {
        Ok(user) => user,
        Err(e) => {
            tracing::error!(user_id = %login.session.user_id, "Failed to fetch user: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch user record"
                })),
            )
                .into_response();
        }
    };

    tracing::debug!(user_id = %user.id, "Updating user with RWGPS connection");
    user.rwgps_connection = Some(rwgps_connection);

    tracing::info!(user_id = %user.id, "Saving updated user");
    if let Err(e) = user_repo.put(user.clone()).await {
        tracing::error!(user_id = %user.id, "Failed to save user with RWGPS connection: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to save RWGPS connection"
            })),
        )
            .into_response();
    }

    tracing::info!(user_id = %user.id, "Enqueueing RWGPS history sync job");
    if let Err(e) = job_storage
        .push(Job::from(RwgpsJob::SyncHistory {
            connection: user.rwgps_connection.unwrap(),
        }))
        .await
    {
        tracing::error!(user_id = %user.id, "Failed to enqueue RWGPS history sync job: {}", e);
        // Continue with the redirect even if sync job creation fails
    }

    tracing::info!(user_id = %user.id, "RWGPS connection complete, redirecting to workshop page");
    Redirect::to("https://howittplains.net/workshop?rwgps=connected").into_response()
}
