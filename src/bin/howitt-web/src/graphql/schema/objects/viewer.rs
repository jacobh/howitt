use anyhow::anyhow;
use async_graphql::{Context, Object};
use chrono::Utc;
use howitt::services::user::auth::Login;
use url::Url;

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
        let SchemaData {
            rwgps_client_id,
            user_auth_service,
            ..
        } = ctx.data()?;

        let Login { token, .. } = user_auth_service.generate_login(
            self.0.session.user_id,
            Utc::now(),
            chrono::Duration::minutes(20),
        )?;

        let url = Url::parse_with_params(
            "https://ridewithgps.com/oauth/authorize",
            &[
                ("client_id", rwgps_client_id.as_str()),
                (
                    "redirect_uri",
                    "https://api.howittplains.net/auth/rwgps/callback",
                ),
                ("response_type", "code"),
                ("state", token.as_str()),
            ],
        )?;

        Ok(url.to_string())
    }
}
