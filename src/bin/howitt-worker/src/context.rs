use std::sync::Arc;

use howitt_client_types::BucketName;
use howitt_clients::S3BucketClient;
use howitt_postgresql::{
    PostgresClient, PostgresMediaRepo, PostgresRidePointsRepo, PostgresRideRepo,
};

#[derive(Clone)]
pub struct Context {
    pub media_repo: Arc<PostgresMediaRepo>,
    pub ride_repo: Arc<PostgresRideRepo>,
    pub ride_points_repo: Arc<PostgresRidePointsRepo>,
    pub bucket_client: Arc<S3BucketClient>,
}

impl Context {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let postgres_client = PostgresClient::connect(
            &std::env::var("DATABASE_URL")
                .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
        )
        .await?;

        let bucket_client = S3BucketClient::new_from_env(BucketName::Media);

        Ok(Self {
            media_repo: Arc::new(PostgresMediaRepo::new(postgres_client.clone())),
            ride_repo: Arc::new(PostgresRideRepo::new(postgres_client.clone())),
            ride_points_repo: Arc::new(PostgresRidePointsRepo::new(postgres_client.clone())),

            bucket_client: Arc::new(bucket_client),
        })
    }
}
