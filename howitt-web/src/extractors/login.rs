use std::convert::Infallible;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    response::IntoResponse,
};
use howitt::services::user::auth::Login;
use http::{header, request::Parts, StatusCode};

use crate::{app_state::AppState, graphql::credentials::Credentials};

impl OptionalFromRequestParts<AppState> for Login {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Login>, Infallible> {
        let AppState {
            user_auth_service, ..
        } = state;

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let credentials = auth_header.and_then(|s| Credentials::parse_auth_header_value(s).ok());

        match credentials {
            Some(Credentials::BearerToken(token)) => match user_auth_service.verify(&token).await {
                Ok(login) => Ok(Some(login)),
                Err(_) => Ok(None),
            },
            Some(Credentials::Key(_)) => Ok(None),
            None => Ok(None),
        }
    }
}

// Add this: Define the AuthError type
#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
    InvalidToken,
    InternalError,
}

// Implement IntoResponse so it can be used as a Rejection
impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (status, message).into_response()
    }
}

impl FromRequestParts<AppState> for Login {
    type Rejection = AuthError; // Custom error type

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Reuse the optional implementation
        let login = <Login as OptionalFromRequestParts<_>>::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError::InternalError)?;

        match login {
            Some(login) => Ok(login),
            None => Err(AuthError::Unauthorized),
        }
    }
}
