use std::sync::Arc;

use crate::checkpoint::Checkpoint;

pub trait Repo<T: Sized, E: Sized>: Send + Sync {
    async fn all(&self) -> Result<Vec<T>, E>;
}

pub type CheckpointRepo = Arc<dyn Repo<Checkpoint, anyhow::Error>>;