use std::sync::Arc;

use apalis_redis::RedisStorage;
use howitt::{jobs::Job, services::user::auth::UserAuthService};
use howitt_clients::S3BucketClient;
use howitt_postgresql::PostgresMediaRepo;

use crate::graphql::schema::Schema;

#[derive(axum_macros::FromRef, Clone)]
pub struct AppState {
    pub schema: Schema,
    pub user_auth_service: UserAuthService,
    pub bucket_client: Arc<S3BucketClient>,
    pub media_repo: Arc<PostgresMediaRepo>,
    pub job_storage: Arc<tokio::sync::Mutex<RedisStorage<Job>>>,
}
