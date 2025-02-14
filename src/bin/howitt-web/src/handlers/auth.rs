use axum::{extract::State, Json};
use chrono::Duration;
use howitt::services::user::{auth::UserAuthService, signup::UserSignupService};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginParams {
    username: String,
    password: String,
}

pub async fn login_handler(
    State(auth_service): State<UserAuthService>,
    Json(params): Json<LoginParams>,
) -> Json<serde_json::Value> {
    let res = auth_service.login(&params.username, &params.password).await;

    match res {
        Ok(Ok(login)) => Json(serde_json::to_value(login).unwrap()),
        _ => Json(serde_json::json!({ "err": "login failed" })),
    }
}

#[derive(Deserialize)]
pub struct SignupParams {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    error: Option<String>,
    token: Option<String>,
}

pub async fn signup_handler(
    State(signup_service): State<UserSignupService>,
    State(auth_service): State<UserAuthService>,
    Json(params): Json<SignupParams>,
) -> Json<SignupResponse> {
    match signup_service
        .signup(params.username, params.email, params.password)
        .await
    {
        Ok(user) => {
            // Generate login token
            match auth_service.generate_login(user.id, chrono::Utc::now(), Duration::days(365)) {
                Ok(login) => Json(SignupResponse {
                    error: None,
                    token: Some(login.token.into()),
                }),
                Err(_) => Json(SignupResponse {
                    error: Some("Failed to generate auth token".to_string()),
                    token: None,
                }),
            }
        }
        Err(err) => Json(SignupResponse {
            error: Some(err.to_string()),
            token: None,
        }),
    }
}
