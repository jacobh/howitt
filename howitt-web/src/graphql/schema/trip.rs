use async_graphql::Object;
use howitt::models::trip::TripId;

use super::ModelId;

pub struct Trip(howitt::models::trip::Trip);

#[Object]
impl Trip {
    async fn id(&self) -> ModelId<TripId> {
        ModelId::from(self.0.id)
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
}
