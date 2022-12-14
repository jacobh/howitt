#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use std::sync::Arc;

use reqwest::{RequestBuilder, Url};
use thiserror::Error;

pub mod config;
pub mod credentials;
mod reqwest_ext;
pub mod types;

use credentials::Credentials;
use reqwest_ext::{ResponseExt, SerdeDebugError};
use tokio::sync::Semaphore;

#[derive(Error, Debug)]
#[error("RWGPS API Error")]
pub enum RwgpsError {
    Reqwest(#[from] reqwest::Error),
    Url(#[from] url::ParseError),
    SerdeDebug(#[from] SerdeDebugError),
}

#[derive(Clone)]
pub struct RwgpsClient {
    client: reqwest::Client,
    base_url: Url,
    credentials: Credentials,
    semaphore: Arc<Semaphore>,
}
impl RwgpsClient {
    pub fn new(credentials: Credentials) -> RwgpsClient {
        RwgpsClient {
            client: reqwest::Client::new(),
            semaphore: Arc::new(Semaphore::new(20)),
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
        let _permit = self.semaphore.acquire().await.unwrap();

        let resp: types::AuthenticatedUserDetailResponse = self
            .get("/users/current.json")?
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp)
    }

    pub async fn user_routes(
        &self,
        user_id: usize,
    ) -> Result<Vec<types::RouteSummary>, RwgpsError> {
        let _permit = self.semaphore.acquire().await.unwrap();

        let resp: types::ListResponse<types::RouteSummary> = self
            .get(&format!("/users/{}/routes.json", user_id))?
            .query(&[("limit", "1000")])
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.results)
    }

    pub async fn route(&self, route_id: usize) -> Result<types::Route, RwgpsError> {
        let _permit = self.semaphore.acquire().await.unwrap();

        let resp: types::RouteResponse = self
            .get(&format!("/routes/{}.json", route_id))?
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.route)
    }
}
