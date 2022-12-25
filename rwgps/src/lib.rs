use reqwest::Url;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("RWGPS API Error")]
pub enum RwgpsError {
    Reqwest(#[from] reqwest::Error),
    Url(#[from] url::ParseError),
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

    pub async fn auth(&self, email: &str, password: &str) -> Result<AuthenticatedUserDetailResponse, RwgpsError> {
        let resp: AuthenticatedUserDetailResponse = self
            .client
            .get(self.base_url.join("/users/current.json")?)
            .query(&[
                ("email", email),
                ("password", password),
                ("apikey", "howitt"),
                ("version", "2"),
            ])
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