use axum::{extract::State, Json};
use howitt::services::user::auth::UserAuthService;
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
