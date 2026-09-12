use anyhow::anyhow;
use async_graphql::{Context, Object};
use howitt::services::user::auth::Login;

use crate::graphql::context::SchemaData;

use super::{user::UserProfile, user_rwgps_connection::UserRwgpsConnection};
pub struct Viewer(pub Login);

#[Object]
impl Viewer {
    async fn id(&self) -> String {
        self.0.session.user_id.to_string()
    }

    async fn profile<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<UserProfile, async_graphql::Error> {
        let SchemaData { user_loader, .. } = ctx.data()?;

        let user = user_loader
            .load_one(self.0.session.user_id)
            .await?
            .ok_or(anyhow!("User not found"))?;

        Ok(UserProfile(user))
    }

    async fn rwgps_connection<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Option<UserRwgpsConnection>, async_graphql::Error> {
        let SchemaData { user_loader, .. } = ctx.data()?;

        let user = user_loader
            .load_one(self.0.session.user_id)
            .await?
            .ok_or(anyhow!("User not found"))?;

        Ok(user.rwgps_connection.map(UserRwgpsConnection))
    }

    async fn rwgps_auth_request_url<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<String, async_graphql::Error> {
        let _ = ctx;
        Err(crate::handlers::disabled::graphql_error())
    }
}
