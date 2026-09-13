use std::sync::Arc;

use crate::models::{
    point::{ElevationPoint, TemporalElevationPoint},
    ride::Ride,
    route::Route,
};

pub type RwgpsSyncResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[async_trait::async_trait]
pub trait RwgpsSyncStore: Send + Sync {
    async fn save_route(&self, route: Route, points: Vec<ElevationPoint>) -> RwgpsSyncResult;

    async fn save_trip(&self, ride: Ride, points: Vec<TemporalElevationPoint>) -> RwgpsSyncResult;
}

pub type DynRwgpsSyncStore = Arc<dyn RwgpsSyncStore>;
