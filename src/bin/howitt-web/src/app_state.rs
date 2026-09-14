use crate::graphql::schema::Schema;
use crate::jobs::DynJobQueue;
use howitt::repos::Repos;
use howitt::services::user::{auth::UserAuthService, signup::UserSignupService};

#[derive(Clone)]
pub struct RwgpsConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Clone)]
pub struct RwgpsWebhookState {
    pub client_secret: String,
    pub jobs: DynJobQueue,
}

#[derive(axum_macros::FromRef, Clone)]
pub struct AppState {
    pub schema: Schema,
    pub user_auth_service: UserAuthService,
    pub user_signup_service: UserSignupService,
    pub repos: Repos,
    pub jobs: DynJobQueue,
    pub rwgps: RwgpsConfig,
    pub images: Option<crate::handlers::media::ImagesConfig>,
}
