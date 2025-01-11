use async_graphql::Object;
use howitt::models::user::UserId;

use super::ModelId;

pub struct UserProfile(pub howitt::models::user::User);

#[Object]
impl UserProfile {
    async fn id(&self) -> ModelId<UserId> {
        ModelId::from(self.0.id)
    }
    async fn username(&self) -> &str {
        &self.0.username
    }
}
