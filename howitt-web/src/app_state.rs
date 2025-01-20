use howitt::services::user::auth::UserAuthService;

use crate::graphql::schema::Schema;

#[derive(axum_macros::FromRef, Clone)]
pub struct AppState {
    pub schema: Schema,
    pub user_auth_service: UserAuthService,
}
