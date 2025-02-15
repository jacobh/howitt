#![allow(incomplete_features)]

use std::sync::Arc;

use reqwest::Url;
use rwgps_types::credentials::Credentials;
use thiserror::Error;

mod reqwest_ext;

use reqwest_ext::{ResponseExt, SerdeDebugError};
use tokio::sync::{Semaphore, SemaphorePermit};

#[derive(Error, Debug)]
#[error("RWGPS API Error {:?}", _0)]
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

    async fn make_request<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: Option<&T>,
        credentials: Option<&Credentials>,
        method: reqwest::Method,
    ) -> Result<R, RwgpsError> {
        let _permit = self.acquire_semaphore_permit().await;

        let url = self.base_url.join(path)?;

        let mut request = self.client.request(method.clone(), url);

        if let Some(creds) = credentials {
            match creds {
                Credentials::Token(token_creds) => {
                    request = request.header(
                        "Authorization",
                        format!("Bearer {}", token_creds.auth_token),
                    );
                }
                _ => {
                    request = request.query(&creds.to_query());
                }
            }
        }

        if let Some(p) = params {
            request = match method {
                reqwest::Method::GET => request.query(p),
                _ => request.json(p),
            };
        }

        let response = request.send().await?.json_debug().await?;

        Ok(response)
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

    pub async fn user_info(
        &self,
    ) -> Result<rwgps_types::AuthenticatedUserDetailResponse, RwgpsError> {
        self.client
            .make_request(
                "/users/current.json",
                None::<&()>,
                Some(&self.credentials),
                reqwest::Method::GET,
            )
            .await
    }

    pub async fn user_routes(
        &self,
        user_id: usize,
    ) -> Result<Vec<rwgps_types::RouteSummary>, RwgpsError> {
        let response: rwgps_types::ListResponse<rwgps_types::RouteSummary> = self
            .client
            .make_request(
                &format!("/users/{user_id}/routes.json"),
                Some(&[("limit", "1000")]),
                Some(&self.credentials),
                reqwest::Method::GET,
            )
            .await?;

        Ok(response.results)
    }

    pub async fn user_trips(
        &self,
        user_id: usize,
    ) -> Result<Vec<rwgps_types::TripSummary>, RwgpsError> {
        let response: rwgps_types::ListResponse<rwgps_types::TripSummary> = self
            .client
            .make_request(
                &format!("/users/{user_id}/trips.json"),
                Some(&[("limit", "5000")]),
                Some(&self.credentials),
                reqwest::Method::GET,
            )
            .await?;

        Ok(response.results)
    }

    pub async fn route(&self, route_id: usize) -> Result<rwgps_types::Route, RwgpsError> {
        let response: rwgps_types::RouteResponse = self
            .client
            .make_request(
                &format!("/routes/{route_id}.json"),
                None::<&()>,
                Some(&self.credentials),
                reqwest::Method::GET,
            )
            .await?;

        Ok(response.route)
    }

    pub async fn trip(&self, trip_id: usize) -> Result<rwgps_types::Trip, RwgpsError> {
        let response: rwgps_types::TripResponse = self
            .client
            .make_request(
                &format!("/trips/{trip_id}.json"),
                None::<&()>,
                Some(&self.credentials),
                reqwest::Method::GET,
            )
            .await?;

        Ok(response.trip)
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
