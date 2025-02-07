use std::sync::Arc;

use howitt::{jobs::Job, repos::Repos, services::user::auth::UserAuthService};
use howitt_clients::S3BucketClient;
use howitt_jobs::storage::LockFreeStorage;

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
    pub repos: Repos,
    pub job_storage: LockFreeStorage<Job>,
    pub rwgps: RwgpsConfig,
}
