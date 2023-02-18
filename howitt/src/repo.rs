use async_trait::async_trait;
use std::sync::Arc;

use crate::{checkpoint::Checkpoint, config::Config, model::Model, route::RouteModel};

#[async_trait]
pub trait Repo<T: Model + Sized, E: Sized>: Send + Sync {
    async fn all(&self) -> Result<Vec<T>, E>;
    async fn get(&self, id: T::Id) -> Result<Option<T>, E>;
}

pub type ConfigRepo = Arc<dyn Repo<Config, anyhow::Error>>;
pub type CheckpointRepo = Arc<dyn Repo<Checkpoint, anyhow::Error>>;
pub type RouteModelRepo = Arc<dyn Repo<RouteModel, anyhow::Error>>;
