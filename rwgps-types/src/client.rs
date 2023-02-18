use std::error::Error;

use async_trait::async_trait;

use crate::*;

#[async_trait]
pub trait RwgpsClient<E: Error>: Clone {
    async fn user_info(&self) -> Result<AuthenticatedUserDetailResponse, E>;

    async fn user_routes(&self, user_id: usize) -> Result<Vec<RouteSummary>, E>;

    async fn user_trips(&self, user_id: usize) -> Result<Vec<TripSummary>, E>;

    async fn route(&self, route_id: usize) -> Result<Route, E>;

    async fn trip(&self, trip_id: usize) -> Result<Trip, E>;
}
