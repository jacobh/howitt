use reqwest::Url;
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

    pub async fn auth(&self, email: &str, password: &str) -> Result<AuthResponse, RwgpsError> {
        let resp: serde_json::Value = self
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

        Ok(AuthResponse {
            auth_token: resp
                .get("user")
                .unwrap()
                .get("auth_token")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        })
    }
}

pub struct AuthResponse {
    pub auth_token: String,
}
