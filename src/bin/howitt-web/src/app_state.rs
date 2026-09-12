use crate::graphql::schema::Schema;
use howitt::services::user::{auth::UserAuthService, signup::UserSignupService};

#[derive(axum_macros::FromRef, Clone)]
pub struct AppState {
    pub schema: Schema,
    pub user_auth_service: UserAuthService,
    pub user_signup_service: UserSignupService,
}
