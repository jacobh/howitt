use async_graphql::{Context, Object};
use howitt::models::{ride::RideFilter, trip::TripId};

use crate::graphql::context::SchemaData;
use crate::graphql::schema::{ride::Ride, ModelId};

pub struct Trip(pub howitt::models::trip::Trip);

#[Object]
impl Trip {
    async fn id(&self) -> ModelId<TripId> {
        ModelId::from(self.0.id)
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn rides<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData { ride_repo, .. } = ctx.data()?;

        let rides = ride_repo
            .filter_models(RideFilter::ForTrip(self.0.id))
            .await?;

        Ok(rides.into_iter().map(Ride).collect())
    }
}
