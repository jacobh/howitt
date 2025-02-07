use std::sync::Arc;

use apalis_redis::RedisStorage;
use howitt::{jobs::Job, repos::Repos};
use howitt_client_types::BucketName;
use howitt_clients::S3BucketClient;
use howitt_jobs::storage::LockFreeStorage;
use howitt_postgresql::{PostgresClient, PostgresRepos};
use rwgps::RwgpsClient;

#[derive(Clone)]
pub struct Context {
    pub repos: Repos,
    pub bucket_client: Arc<S3BucketClient>,
    pub rwgps_client: RwgpsClient,
    pub image_processing_semaphore: Arc<tokio::sync::Semaphore>,
    pub job_storage: LockFreeStorage<Job>,
}

impl Context {
    pub async fn new(job_storage: RedisStorage<Job>) -> Result<Self, anyhow::Error> {
        let postgres_client = PostgresClient::connect(
            &std::env::var("DATABASE_URL")
                .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
        )
        .await?;

        let bucket_client = S3BucketClient::new_from_env(BucketName::Media);

        Ok(Self {
            repos: Repos::from(PostgresRepos::new(postgres_client)),
            bucket_client: Arc::new(bucket_client),
            rwgps_client: RwgpsClient::new(),
            image_processing_semaphore: Arc::new(tokio::sync::Semaphore::new(4)),
            job_storage: LockFreeStorage::new(job_storage),
        })
    }
}
