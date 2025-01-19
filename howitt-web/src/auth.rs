use axum::{extract::State, routing::post, Json, Router};
use howitt::services::user::auth::UserAuthService;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoginParams {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    err: &'static str,
}

async fn login_handler(
    State(auth_service): State<UserAuthService>,
    Json(params): Json<LoginParams>,
) -> Json<serde_json::Value> {
    let res = auth_service.login(&params.username, &params.password).await;

    match res {
        Ok(Ok(login)) => Json(serde_json::to_value(login).unwrap()),
        _ => Json(serde_json::json!({ "err": "login failed" })),
    }
}

pub fn login_routes(auth_service: UserAuthService) -> Router {
    Router::new()
        .route("/auth/login", post(login_handler))
        .with_state(auth_service)
}
