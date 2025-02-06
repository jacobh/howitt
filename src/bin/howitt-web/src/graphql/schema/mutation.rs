use std::collections::HashSet;

use async_graphql::*;
use chrono::{DateTime, Utc};
use howitt::models::media::MediaId;
use howitt::models::trip::{TripId, TripNote};
use howitt::repos::Repos;
use itertools::Itertools;

use crate::graphql::context::{RequestData, SchemaData};
use crate::graphql::schema::{trip::Trip, ModelId};

use super::viewer::Viewer;

#[derive(InputObject)]
pub struct TripNoteInput {
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(InputObject)]
pub struct UpdateTripInput {
    pub trip_id: ModelId<TripId>,
    pub name: String,
    pub description: Option<String>,
    pub notes: Vec<TripNoteInput>,
}

#[derive(SimpleObject)]
pub struct UpdateTripOutput {
    pub trip: Option<Trip>,
}

#[derive(InputObject)]
pub struct AddTripMediaInput {
    pub trip_id: ModelId<TripId>,
    pub media_ids: Vec<ModelId<MediaId>>,
}

#[derive(InputObject)]
pub struct RemoveTripMediaInput {
    pub trip_id: ModelId<TripId>,
    pub media_ids: Vec<ModelId<MediaId>>,
}

#[derive(SimpleObject)]
pub struct TripMediaOutput {
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
        let SchemaData {
            repos: Repos { trip_repo, .. },
            ..
        } = ctx.data()?;
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
        trip.notes = input
            .notes
            .into_iter()
            .map(|note| TripNote {
                timestamp: note.timestamp,
                text: note.text,
            })
            .collect();

        // Save changes
        trip_repo.put(trip.clone()).await?;

        Ok(UpdateTripOutput {
            trip: Some(Trip(trip)),
        })
    }

    async fn add_trip_media(
        &self,
        ctx: &Context<'_>,
        input: AddTripMediaInput,
    ) -> Result<TripMediaOutput, Error> {
        // Get required context data
        let SchemaData {
            repos: Repos { trip_repo, .. },
            ..
        } = ctx.data()?;
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

        let media_ids = trip
            .media_ids
            .clone()
            .into_iter()
            .chain(input.media_ids.into_iter().map(|id| id.0))
            .unique()
            .collect_vec();

        trip.media_ids = media_ids;

        // Save changes
        trip_repo.put(trip.clone()).await?;

        Ok(TripMediaOutput {
            trip: Some(Trip(trip)),
        })
    }

    async fn remove_trip_media(
        &self,
        ctx: &Context<'_>,
        input: RemoveTripMediaInput,
    ) -> Result<TripMediaOutput, Error> {
        // Get required context data
        let SchemaData {
            repos: Repos { trip_repo, .. },
            ..
        } = ctx.data()?;
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

        let media_ids_to_remove: HashSet<MediaId> =
            HashSet::from_iter(input.media_ids.into_iter().map(|id| id.0));

        let media_ids = trip
            .media_ids
            .clone()
            .into_iter()
            .filter(|id| !media_ids_to_remove.contains(id))
            .collect_vec();

        trip.media_ids = media_ids;

        // Save changes
        trip_repo.put(trip.clone()).await?;

        Ok(TripMediaOutput {
            trip: Some(Trip(trip)),
        })
    }

    async fn clear_rwgps_connection(&self, ctx: &Context<'_>) -> Result<Viewer, Error> {
        // Get required context data
        let SchemaData {
            repos: Repos { user_repo, .. },
            ..
        } = ctx.data()?;
        let RequestData { login } = ctx.data()?;

        // Ensure user is logged in
        let login = login
            .as_ref()
            .ok_or_else(|| Error::new("Authentication required"))?
            .clone();

        // Get the user
        let mut user = user_repo.get(login.session.user_id).await?;

        // Clear the RWGPS connection
        user.rwgps_connection = None;

        // Save changes
        user_repo.put(user).await?;

        Ok(Viewer(login))
    }
}
