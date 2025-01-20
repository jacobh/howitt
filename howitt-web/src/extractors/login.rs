use std::convert::Infallible;

use axum::extract::OptionalFromRequestParts;
use howitt::services::user::auth::Login;
use http::{header, request::Parts};

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
