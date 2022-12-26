use reqwest::{RequestBuilder, Url};
use thiserror::Error;

pub mod credentials;
pub mod types;

use credentials::Credentials;

#[derive(Error, Debug)]
#[error("RWGPS API Error")]
pub enum RwgpsError {
    Reqwest(#[from] reqwest::Error),
    Url(#[from] url::ParseError),
}

#[derive(Clone)]
pub struct RwgpsClient {
    client: reqwest::Client,
    base_url: Url,
    credentials: Credentials,
}
impl RwgpsClient {
    pub fn new(credentials: Credentials) -> RwgpsClient {
        RwgpsClient {
            client: reqwest::Client::new(),
            base_url: Url::parse("https://ridewithgps.com").unwrap(),
            credentials,
        }
    }

    fn get(&self, path: &str) -> Result<RequestBuilder, RwgpsError> {
        Ok(self
            .client
            .get(self.base_url.join(path)?)
            .query(&self.credentials.to_query()))
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
    ) -> Result<Vec<types::RouteSummary>, RwgpsError> {
        let resp: types::ListResponse<types::RouteSummary> = self
            .get(&format!("/users/{}/routes.json", user_id))?
            .query(&[("limit", "1000")])
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.results)
    }

    pub async fn route(&self, route_id: usize) -> Result<types::Route, RwgpsError> {
        let resp: types::RouteResponse = self
            .get(&format!("/routes/{}.json", route_id))?
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.route)
    }
}
