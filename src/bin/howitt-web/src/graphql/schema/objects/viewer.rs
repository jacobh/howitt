use async_graphql::{Context, Object};
use howitt::services::user::auth::Login;

use crate::graphql::context::SchemaData;

use super::user::UserProfile;

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
        let SchemaData { user_repo, .. } = ctx.data()?;

        let user = user_repo.get(self.0.session.user_id).await?;

        Ok(UserProfile(user))
    }
}
