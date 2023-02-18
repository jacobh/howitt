use async_trait::async_trait;
use std::sync::Arc;

use crate::models::{
    checkpoint::Checkpoint, config::Config, ride::RideModel, route::RouteModel, Model,
};

#[async_trait]
pub trait Repo<T: Model + Sized, E: Sized>: Send + Sync {
    async fn all(&self) -> Result<Vec<T>, E>;
    async fn get(&self, id: T::Id) -> Result<Option<T>, E>;
    async fn get_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T>, E>;
}

pub type ConfigRepo = Arc<dyn Repo<Config, anyhow::Error>>;
pub type CheckpointRepo = Arc<dyn Repo<Checkpoint, anyhow::Error>>;
pub type RouteModelRepo = Arc<dyn Repo<RouteModel, anyhow::Error>>;
pub type RideModelRepo = Arc<dyn Repo<RideModel, anyhow::Error>>;
