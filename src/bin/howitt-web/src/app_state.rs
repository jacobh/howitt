use std::sync::Arc;

use apalis_redis::RedisStorage;
use howitt::{jobs::Job, services::user::auth::UserAuthService};
use howitt_clients::S3BucketClient;
use howitt_postgresql::{PostgresMediaRepo, PostgresUserRepo};

use crate::graphql::schema::Schema;

#[derive(Clone)]
pub struct RwgpsConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(axum_macros::FromRef, Clone)]
pub struct AppState {
    pub schema: Schema,
    pub user_auth_service: UserAuthService,
    pub bucket_client: Arc<S3BucketClient>,
    pub media_repo: Arc<PostgresMediaRepo>,
    pub user_repo: Arc<PostgresUserRepo>,
    pub job_storage: Arc<tokio::sync::Mutex<RedisStorage<Job>>>,
    pub rwgps: RwgpsConfig,
}
