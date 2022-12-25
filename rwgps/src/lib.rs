use reqwest::{Url, RequestBuilder};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("RWGPS API Error")]
pub enum RwgpsError {
    Reqwest(#[from] reqwest::Error),
    Url(#[from] url::ParseError),
}

#[derive(derive_more::From, Debug, Clone, Serialize, Deserialize)]
pub enum AuthInfo {
    Password(PasswordAuthInfo),
    Token(TokenAuthInfo)
}
impl AuthInfo {
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
            })
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordAuthInfo {
    pub email: String,
    pub password: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAuthInfo {
    pub auth_token: String
}

pub struct RwgpsClient {
    client: reqwest::Client,
    base_url: Url,
}
impl RwgpsClient {
    pub fn new() -> RwgpsClient {
        RwgpsClient {
            client: reqwest::Client::new(),
            base_url: Url::parse("https://ridewithgps.com").unwrap(),
        }
    }

    fn get(&self, path: &str) -> Result<RequestBuilder, RwgpsError> {
        Ok(self.client.get(self.base_url.join(path)?))
    }

    pub async fn user_info(&self, auth_info: &AuthInfo) -> Result<AuthenticatedUserDetailResponse, RwgpsError> {
        let resp: AuthenticatedUserDetailResponse = self
            .get("/users/current.json")?
            .query(&auth_info.to_query())
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }
}

#[derive(Deserialize)]
pub struct AuthenticatedUserDetailResponse {
    pub user: UserInfo
}

#[derive(Deserialize)]
pub struct UserInfo {
    pub auth_token: String,
    pub id: usize
}