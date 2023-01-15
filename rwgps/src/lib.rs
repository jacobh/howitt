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
use tokio::sync::{Semaphore, SemaphorePermit};

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

    async fn acquire_semaphore_permit(&self) -> SemaphorePermit {
        self.semaphore.acquire().await.unwrap()
    }

    pub async fn user_info(&self) -> Result<types::AuthenticatedUserDetailResponse, RwgpsError> {
        let _permit = self.acquire_semaphore_permit().await;

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
        let _permit = self.acquire_semaphore_permit().await;

        let resp: types::ListResponse<types::RouteSummary> = self
            .get(&format!("/users/{}/routes.json", user_id))?
            .query(&[("limit", "1000")])
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.results)
    }

    pub async fn user_trips(&self, user_id: usize) -> Result<Vec<types::TripSummary>, RwgpsError> {
        let _permit = self.acquire_semaphore_permit().await;

        let resp: types::ListResponse<types::TripSummary> = self
            .get(&format!("/users/{}/trips.json", user_id))?
            .query(&[("limit", "5000")])
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.results)
    }

    pub async fn route(&self, route_id: usize) -> Result<types::Route, RwgpsError> {
        let _permit = self.acquire_semaphore_permit().await;

        let resp: types::RouteResponse = self
            .get(&format!("/routes/{}.json", route_id))?
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.route)
    }

    pub async fn trip(&self, trip_id: usize) -> Result<types::Trip, RwgpsError> {
        let _permit = self.acquire_semaphore_permit().await;

        let resp: types::TripResponse = self
            .get(&format!("/trips/{}.json", trip_id))?
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.trip)
    }
}
