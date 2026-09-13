use async_trait::async_trait;
use howitt::jobs::Job;
#[cfg(target_arch = "wasm32")]
use howitt::jobs::QueueMessage;
use std::sync::Arc;

#[async_trait]
pub trait JobQueue: Send + Sync {
    async fn publish(&self, jobs: Vec<Job>) -> anyhow::Result<()>;
}

pub type DynJobQueue = Arc<dyn JobQueue>;

#[cfg(target_arch = "wasm32")]
pub struct CloudflareJobQueue(worker::Queue);

#[cfg(target_arch = "wasm32")]
impl CloudflareJobQueue {
    pub fn new(queue: worker::Queue) -> Self {
        Self(queue)
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait]
impl JobQueue for CloudflareJobQueue {
    async fn publish(&self, jobs: Vec<Job>) -> anyhow::Result<()> {
        if jobs.is_empty() {
            return Ok(());
        }

        worker::send::SendFuture::new(
            self.0
                .send_batch(jobs.into_iter().map(QueueMessage::V1).collect::<Vec<_>>()),
        )
        .await
        .map_err(|error| anyhow::anyhow!(error.to_string()))
    }
}
