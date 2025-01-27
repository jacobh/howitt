use async_graphql::*;
use howitt::models::trip::TripId;

use crate::graphql::context::{RequestData, SchemaData};
use crate::graphql::schema::{trip::Trip, ModelId};

#[derive(InputObject)]
pub struct UpdateTripInput {
    pub trip_id: ModelId<TripId>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(SimpleObject)]
pub struct UpdateTripOutput {
    pub trip: Option<Trip>,
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn update_trip(
        &self,
        ctx: &Context<'_>,
        input: UpdateTripInput,
    ) -> Result<UpdateTripOutput, Error> {
        // Get required context data
        let SchemaData { trip_repo, .. } = ctx.data()?;
        let RequestData { login } = ctx.data()?;

        // Ensure user is logged in
        let login = login
            .as_ref()
            .ok_or_else(|| Error::new("Authentication required"))?;

        // Get the trip
        let mut trip = trip_repo.get(input.trip_id.0).await?;

        // Verify ownership
        if trip.user_id != login.session.user_id {
            return Err(Error::new("Not authorized to update this trip"));
        }

        // Update the fields
        trip.name = input.name;
        trip.description = input.description;

        // Save changes
        trip_repo.put(trip.clone()).await?;

        Ok(UpdateTripOutput {
            trip: Some(Trip(trip)),
        })
    }
}
