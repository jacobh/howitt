use reqwest::{RequestBuilder, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

    pub async fn user_info(
        &self,
        auth_info: &AuthInfo,
    ) -> Result<AuthenticatedUserDetailResponse, RwgpsError> {
        let resp: AuthenticatedUserDetailResponse = self
            .get("/users/current.json")?
            .query(&auth_info.to_query())
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn user_routes(
        &self,
        auth_info: &AuthInfo,
        user_id: usize,
    ) -> Result<ListResponse<RouteSummary>, RwgpsError> {
        let resp: ListResponse<RouteSummary> = self
            .get(&format!("/users/{}/routes.json", user_id))?
            .query(&auth_info.to_query())
            .query(&[("limit", "1000")])
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }
}

#[derive(Deserialize, Debug)]
pub struct AuthenticatedUserDetailResponse {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub auth_token: String,
    pub id: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub results: Vec<T>,
    pub results_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSummary {
    pub administrative_area: String,
    pub archived_at: Value,
    pub best_for_id: Option<i64>,
    pub country_code: String,
    pub created_at: String,
    pub deleted_at: Value,
    pub description: Option<String>,
    pub difficulty: String,
    pub distance: Option<f64>,
    pub elevation_gain: f64,
    pub elevation_loss: f64,
    pub first_lat: f64,
    pub first_lng: f64,
    pub group_membership_id: i64,
    pub has_course_points: bool,
    pub highlighted_photo_checksum: Value,
    pub highlighted_photo_id: i64,
    pub id: i64,
    pub is_trip: bool,
    pub last_lat: f64,
    pub last_lng: f64,
    pub likes_count: i64,
    pub locality: String,
    pub name: String,
    pub ne_lat: f64,
    pub ne_lng: f64,
    pub pavement_type_id: Option<i64>,
    pub planner_options: Option<i64>,
    pub postal_code: Option<String>,
    pub sw_lat: f64,
    pub sw_lng: f64,
    pub terrain: String,
    pub track_id: String,
    pub track_type: String,
    pub unpaved_pct: i64,
    pub updated_at: String,
    pub user_id: i64,
    pub visibility: i64,
}
