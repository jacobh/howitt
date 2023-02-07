use async_trait::async_trait;
use std::sync::Arc;

use crate::{checkpoint::Checkpoint, route::Route};

#[async_trait]
pub trait Repo<T: Sized, E: Sized>: Send + Sync {
    async fn all(self: &Self) -> Result<Vec<T>, E>;
}

pub type CheckpointRepo = Arc<dyn Repo<Checkpoint, anyhow::Error>>;
pub type RouteRepo = Arc<dyn Repo<Route, anyhow::Error>>;