use reqwest::{RequestBuilder, Url};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod types;

#[derive(Error, Debug)]
#[error("RWGPS API Error")]
pub enum RwgpsError {
    Reqwest(#[from] reqwest::Error),
    Url(#[from] url::ParseError),
}

#[derive(derive_more::From, Debug, Clone, Serialize, Deserialize)]
pub enum AuthInfo {
    Password(PasswordAuthInfo),
    Token(TokenAuthInfo),
}
impl AuthInfo {
    pub fn from_token(auth_token: String) -> AuthInfo {
        AuthInfo::Token(TokenAuthInfo { auth_token })
    }
    fn to_query(&self) -> serde_json::Value {
        match self {
            AuthInfo::Password(info) => serde_json::json!({
                "email": info.email,
                "password": info.password,
                "apikey": "howitt",
                "version": "2"
            }),
            AuthInfo::Token(info) => serde_json::json!({
                "auth_token": info.auth_token,
                "apikey": "howitt",
                "version": "2"
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordAuthInfo {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAuthInfo {
    pub auth_token: String,
}

pub struct RwgpsClient {
    client: reqwest::Client,
    base_url: Url,
    auth_info: AuthInfo,
}
impl RwgpsClient {
    pub fn new(auth_info: AuthInfo) -> RwgpsClient {
        RwgpsClient {
            client: reqwest::Client::new(),
            base_url: Url::parse("https://ridewithgps.com").unwrap(),
            auth_info,
        }
    }

    fn get(&self, path: &str) -> Result<RequestBuilder, RwgpsError> {
        Ok(self
            .client
            .get(self.base_url.join(path)?)
            .query(&self.auth_info.to_query()))
    }

    pub async fn user_info(&self) -> Result<types::AuthenticatedUserDetailResponse, RwgpsError> {
        let resp: types::AuthenticatedUserDetailResponse = self
            .get("/users/current.json")?
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn user_routes(
        &self,
        user_id: usize,
    ) -> Result<types::ListResponse<types::RouteSummary>, RwgpsError> {
        let resp: types::ListResponse<types::RouteSummary> = self
            .get(&format!("/users/{}/routes.json", user_id))?
            .query(&[("limit", "1000")])
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }
}

