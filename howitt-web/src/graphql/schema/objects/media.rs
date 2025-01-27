use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use howitt::models::media::MediaId;

use crate::graphql::{context::SchemaData, schema::ModelId};

use super::user::UserProfile;

pub struct Media(pub howitt::models::media::Media);

#[Object]
impl Media {
    async fn id(&self) -> ModelId<MediaId> {
        ModelId::from(self.0.id)
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.0.created_at
    }

    async fn path(&self) -> &str {
        &self.0.path
    }

    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserProfile, async_graphql::Error> {
        let SchemaData { user_loader, .. } = ctx.data()?;

        let user = user_loader
            .load_one(self.0.user_id)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?;

        Ok(UserProfile(user))
    }
}
