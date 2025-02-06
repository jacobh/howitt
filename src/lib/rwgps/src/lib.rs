#![allow(incomplete_features)]

use std::sync::Arc;

use reqwest::{RequestBuilder, Url};
use rwgps_types::credentials::Credentials;
use thiserror::Error;

mod reqwest_ext;

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
    semaphore: Arc<Semaphore>,
}
impl RwgpsClient {
    pub fn new() -> RwgpsClient {
        RwgpsClient {
            client: reqwest::Client::new(),
            semaphore: Arc::new(Semaphore::new(20)),
            base_url: Url::parse("https://ridewithgps.com").unwrap(),
        }
    }

    async fn acquire_semaphore_permit(&self) -> SemaphorePermit {
        self.semaphore.acquire().await.unwrap()
    }
}

#[derive(Clone)]
pub struct AuthenticatedRwgpsClient {
    client: RwgpsClient,
    credentials: Credentials,
}
impl AuthenticatedRwgpsClient {
    pub fn new(credentials: Credentials) -> AuthenticatedRwgpsClient {
        AuthenticatedRwgpsClient {
            client: RwgpsClient::new(),
            credentials,
        }
    }

    fn get(&self, path: &str) -> Result<RequestBuilder, RwgpsError> {
        Ok(self
            .client
            .client
            .get(self.client.base_url.join(path)?)
            .query(&self.credentials.to_query()))
    }

    pub async fn user_info(
        &self,
    ) -> Result<rwgps_types::AuthenticatedUserDetailResponse, RwgpsError> {
        let _permit = self.client.acquire_semaphore_permit().await;

        let resp: rwgps_types::AuthenticatedUserDetailResponse = self
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
    ) -> Result<Vec<rwgps_types::RouteSummary>, RwgpsError> {
        let _permit = self.client.acquire_semaphore_permit().await;

        let resp: rwgps_types::ListResponse<rwgps_types::RouteSummary> = self
            .get(&format!("/users/{user_id}/routes.json"))?
            .query(&[("limit", "1000")])
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.results)
    }

    pub async fn user_trips(
        &self,
        user_id: usize,
    ) -> Result<Vec<rwgps_types::TripSummary>, RwgpsError> {
        let _permit = self.client.acquire_semaphore_permit().await;

        let resp: rwgps_types::ListResponse<rwgps_types::TripSummary> = self
            .get(&format!("/users/{user_id}/trips.json"))?
            .query(&[("limit", "5000")])
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.results)
    }

    pub async fn route(&self, route_id: usize) -> Result<rwgps_types::Route, RwgpsError> {
        let _permit = self.client.acquire_semaphore_permit().await;

        let resp: rwgps_types::RouteResponse = self
            .get(&format!("/routes/{route_id}.json"))?
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.route)
    }

    pub async fn trip(&self, trip_id: usize) -> Result<rwgps_types::Trip, RwgpsError> {
        let _permit = self.client.acquire_semaphore_permit().await;

        let resp: rwgps_types::TripResponse = self
            .get(&format!("/trips/{trip_id}.json"))?
            .send()
            .await?
            .json_debug()
            .await?;

        Ok(resp.trip)
    }
}

#[async_trait::async_trait]
impl rwgps_types::client::RwgpsClient for RwgpsClient {
    type Error = RwgpsError;
    type AuthenticatedClient = AuthenticatedRwgpsClient;

    fn with_credentials(&self, credentials: Credentials) -> Self::AuthenticatedClient {
        AuthenticatedRwgpsClient {
            client: self.clone(),
            credentials,
        }
    }
}

#[async_trait::async_trait]
impl rwgps_types::client::AuthenticatedRwgpsClient for AuthenticatedRwgpsClient {
    type Error = RwgpsError;

    async fn user_info(&self) -> Result<rwgps_types::AuthenticatedUserDetailResponse, RwgpsError> {
        self.user_info().await
    }

    async fn user_routes(
        &self,
        user_id: usize,
    ) -> Result<Vec<rwgps_types::RouteSummary>, RwgpsError> {
        self.user_routes(user_id).await
    }

    async fn user_trips(
        &self,
        user_id: usize,
    ) -> Result<Vec<rwgps_types::TripSummary>, RwgpsError> {
        self.user_trips(user_id).await
    }

    async fn route(&self, route_id: usize) -> Result<rwgps_types::Route, RwgpsError> {
        self.route(route_id).await
    }

    async fn trip(&self, trip_id: usize) -> Result<rwgps_types::Trip, RwgpsError> {
        self.trip(trip_id).await
    }
}
