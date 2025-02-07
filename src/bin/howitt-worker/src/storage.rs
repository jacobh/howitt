use apalis::prelude::*;
use apalis_core::request::Parts;
use apalis_redis::RedisStorage;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Clone)]
pub struct LockFreeStorage<Job> {
    sender: mpsc::Sender<StorageMessage<Job>>,
}

pub struct StorageMessage<Job> {
    job: Job,
    response_tx: oneshot::Sender<PushResult>,
}

type PushResult = Result<Parts<apalis_redis::RedisContext>, apalis_redis::RedisError>;

#[derive(Debug, Error)]
pub enum LockFreeStorageError {
    #[error("Redis error: {0}")]
    Redis(#[from] apalis_redis::RedisError),
    #[error("Tokio channel error: {0}")]
    TokioChannel(anyhow::Error),
}

impl LockFreeStorage<howitt::jobs::Job> {
    pub fn new(mut storage: RedisStorage<howitt::jobs::Job>) -> Self {
        let (tx, mut rx) = mpsc::channel::<StorageMessage<howitt::jobs::Job>>(100);

        // Spawn background task to handle storage operations
        tokio::spawn(async move {
            while let Some(StorageMessage { job, response_tx }) = rx.recv().await {
                let result = storage.push(job).await;
                let _ = response_tx.send(result);
            }
        });

        Self { sender: tx }
    }

    pub async fn push(
        &self,
        job: howitt::jobs::Job,
    ) -> Result<Parts<apalis_redis::RedisContext>, LockFreeStorageError> {
        let (response_tx, response_rx) = oneshot::channel();

        // Send job and response channel to worker task
        self.sender
            .send(StorageMessage { job, response_tx })
            .await
            .map_err(|e| LockFreeStorageError::TokioChannel(e.into()))?;

        // Wait for result
        Ok(response_rx
            .await
            .map_err(|e| LockFreeStorageError::TokioChannel(e.into()))??)
    }
}
