use std::error::Error;

use async_trait::async_trait;

use crate::*;

#[async_trait]
pub trait RwgpsClient: Clone {
    type Error: Error;

    async fn user_info(&self) -> Result<AuthenticatedUserDetailResponse, Self::Error>;

    async fn user_routes(&self, user_id: usize) -> Result<Vec<RouteSummary>, Self::Error>;

    async fn user_trips(&self, user_id: usize) -> Result<Vec<TripSummary>, Self::Error>;

    async fn route(&self, route_id: usize) -> Result<Route, Self::Error>;

    async fn trip(&self, trip_id: usize) -> Result<Trip, Self::Error>;
}
