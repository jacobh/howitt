use async_graphql::Object;
use chrono::{DateTime, Utc};
use howitt::models::user::UserRwgpsConnection as ModelUserRwgpsConnection;
use uuid::Uuid;

pub struct UserRwgpsConnection(pub ModelUserRwgpsConnection);

#[Object]
impl UserRwgpsConnection {
    async fn id(&self) -> Uuid {
        self.0.id
    }

    async fn rwgps_user_id(&self) -> i32 {
        self.0.rwgps_user_id
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.0.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.0.updated_at
    }
}
