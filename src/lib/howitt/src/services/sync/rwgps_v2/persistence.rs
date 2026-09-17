use std::sync::Arc;

use crate::models::{
    point::{ElevationPoint, TemporalElevationPoint},
    ride::Ride,
    route::Route,
};

pub type RwgpsSyncResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, thiserror::Error)]
#[error("RWGPS persistence failed")]
pub struct RwgpsSyncPersistenceError {
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl From<Box<dyn std::error::Error + Send + Sync>> for RwgpsSyncPersistenceError {
    fn from(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self { source }
    }
}

#[async_trait::async_trait]
pub trait RwgpsSyncStore: Send + Sync {
    async fn save_route(&self, route: Route, points: Vec<ElevationPoint>) -> RwgpsSyncResult;

    async fn save_trip(&self, ride: Ride, points: Vec<TemporalElevationPoint>) -> RwgpsSyncResult;
}

pub type DynRwgpsSyncStore = Arc<dyn RwgpsSyncStore>;
