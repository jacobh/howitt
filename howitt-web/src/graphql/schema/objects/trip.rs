use async_graphql::{Context, Object};
use howitt::models::{ride::RideFilter, trip::TripId};

use crate::graphql::context::SchemaData;
use crate::graphql::schema::{ride::Ride, ModelId};

use super::user::UserProfile;

pub struct Trip(pub howitt::models::trip::Trip);
pub struct TripLeg(pub Vec<howitt::models::ride::Ride>);

#[Object]
impl TripLeg {
    async fn rides(&self) -> Vec<Ride> {
        self.0.clone().into_iter().map(Ride).collect()
    }
}

#[Object]
impl Trip {
    async fn id(&self) -> ModelId<TripId> {
        ModelId::from(self.0.id)
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn year(&self) -> i32 {
        self.0.year
    }

    async fn slug(&self) -> &str {
        &self.0.slug
    }

    async fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }

    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserProfile, async_graphql::Error> {
        let SchemaData { user_loader, .. } = ctx.data()?;

        let user = user_loader
            .load_one(self.0.user_id)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?;

        Ok(UserProfile(user))
    }

    async fn legs<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<TripLeg>, async_graphql::Error> {
        let SchemaData { ride_repo, .. } = ctx.data()?;

        let rides = ride_repo
            .filter_models(RideFilter::ForTrip(self.0.id))
            .await?;

        // For this first cut, put all rides in a single leg
        Ok(vec![TripLeg(rides)])
    }
}
