use async_trait::async_trait;
use std::sync::Arc;

use crate::models::{
    checkpoint::Checkpoint, config::Config, ride::RideModel, route::RouteModel, Model,
};

#[async_trait]
pub trait Repo<T: Model + Sized>: Send + Sync {
    type Error: Into<anyhow::Error>;

    async fn all(&self) -> Result<Vec<T>, Self::Error>;
    async fn get(&self, id: T::Id) -> Result<Option<T>, Self::Error>;
    async fn get_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T>, Self::Error>;
}

pub type ConfigRepo = Arc<dyn Repo<Config, Error = anyhow::Error>>;
pub type CheckpointRepo = Arc<dyn Repo<Checkpoint, Error = anyhow::Error>>;
pub type RouteModelRepo = Arc<dyn Repo<RouteModel, Error = anyhow::Error>>;
pub type RideModelRepo = Arc<dyn Repo<RideModel, Error = anyhow::Error>>;
