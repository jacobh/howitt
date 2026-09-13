use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use chrono::Utc;
use howitt::{
    jobs::{Job, rwgps::RwgpsJob},
    models::user::UserRwgpsConnection,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::{AppState, RwgpsConfig};

const CONNECTED_REDIRECT: &str = "https://howittplains.net/workshop?rwgps=connected";
const TOKEN_URL: &str = "https://ridewithgps.com/oauth/token.json";

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'static str,
    code: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    user_id: i64,
}

async fn exchange_code(
    config: &RwgpsConfig,
    code: &str,
    token_url: &str,
) -> anyhow::Result<TokenResponse> {
    let request = reqwest::Client::new().post(token_url).json(&TokenRequest {
        grant_type: "authorization_code",
        code,
        client_id: &config.client_id,
        client_secret: &config.client_secret,
        redirect_uri: &config.redirect_uri,
    });
    #[cfg(target_arch = "wasm32")]
    let token = worker::send::SendFuture::new(async move {
        request
            .send()
            .await?
            .error_for_status()?
            .json::<TokenResponse>()
            .await
    })
    .await?;
    #[cfg(not(target_arch = "wasm32"))]
    let token = request.send().await?.error_for_status()?.json().await?;
    Ok(token)
}

fn error(status: StatusCode, message: &'static str) -> Response {
    (status, Json(serde_json::json!({ "error": message }))).into_response()
}

pub async fn handler(
    State(state): State<AppState>,
    Query(params): Query<CallbackParams>,
) -> Response {
    let login = match state.user_auth_service.verify(&params.state).await {
        Ok(login) => login,
        Err(_) => return error(StatusCode::BAD_REQUEST, "Invalid authentication state"),
    };
    let token = match exchange_code(&state.rwgps, &params.code, TOKEN_URL).await {
        Ok(token) => token,
        Err(_) => {
            return error(
                StatusCode::BAD_REQUEST,
                "Failed to exchange authorization code",
            );
        }
    };
    let rwgps_user_id = match i32::try_from(token.user_id) {
        Ok(user_id) => user_id,
        Err(_) => return error(StatusCode::BAD_REQUEST, "Invalid RWGPS user ID"),
    };
    let mut user = match state.repos.user_repo.get(login.session.user_id).await {
        Ok(user) => user,
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch user record",
            );
        }
    };
    let now = Utc::now();
    user.rwgps_connection = Some(UserRwgpsConnection {
        id: Uuid::new_v4(),
        user_id: user.id,
        rwgps_user_id,
        access_token: token.access_token,
        created_at: now,
        updated_at: now,
    });
    if state.repos.user_repo.put(user).await.is_err() {
        return error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save RWGPS connection",
        );
    }

    if state
        .jobs
        .publish(vec![Job::Rwgps(RwgpsJob::SyncHistory {
            user_id: login.session.user_id,
        })])
        .await
        .is_err()
    {
        tracing::error!("Failed to enqueue initial RWGPS history sync");
    }

    Redirect::to(CONNECTED_REDIRECT).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
    };

    #[tokio::test]
    async fn exchanges_code_with_the_configured_oauth_values() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/oauth/token.json", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                .unwrap();
            let mut bytes = Vec::new();
            let header_end = loop {
                let mut byte = [0];
                socket.read_exact(&mut byte).unwrap();
                bytes.push(byte[0]);
                if bytes.ends_with(b"\r\n\r\n") {
                    break bytes.len();
                }
            };
            let headers = String::from_utf8(bytes.clone()).unwrap().to_lowercase();
            let length: usize = headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length: "))
                .unwrap()
                .parse()
                .unwrap();
            bytes.resize(header_end + length, 0);
            socket.read_exact(&mut bytes[header_end..]).unwrap();
            let body: serde_json::Value = serde_json::from_slice(&bytes[header_end..]).unwrap();
            assert_eq!(
                body,
                serde_json::json!({
                    "grant_type": "authorization_code",
                    "code": "one-time-code",
                    "client_id": "client-id",
                    "client_secret": "client-secret",
                    "redirect_uri": "https://api.example.test/auth/rwgps/callback"
                })
            );
            let response = r#"{"access_token":"access-token","user_id":42}"#;
            write!(
                socket,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}",
                response.len()
            )
            .unwrap();
        });

        let token = exchange_code(
            &RwgpsConfig {
                client_id: "client-id".into(),
                client_secret: "client-secret".into(),
                redirect_uri: "https://api.example.test/auth/rwgps/callback".into(),
            },
            "one-time-code",
            &url,
        )
        .await
        .unwrap();
        assert_eq!(token.access_token, "access-token");
        assert_eq!(token.user_id, 42);
        server.join().unwrap();
    }
}
