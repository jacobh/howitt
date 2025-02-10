use async_graphql::*;
use chrono::{DateTime, Datelike, Utc};
use howitt::jobs::rwgps::RwgpsJob;
use howitt::jobs::Job;
use howitt::models::media::MediaId;
use howitt::models::ride::{RideFilter, RideId};
use howitt::models::trip::{Trip as TripModel, TripId, TripNote};
use howitt::repos::Repos;
use howitt::services::slug::generate_slug;

use crate::graphql::context::{RequestData, SchemaData};
use crate::graphql::schema::{trip::Trip, ModelId};

use super::viewer::Viewer;

#[derive(InputObject)]
pub struct CreateTripInput {
    pub name: String,
    pub ride_ids: Vec<ModelId<RideId>>,
    pub description: Option<String>,
}

#[derive(SimpleObject)]
pub struct CreateTripOutput {
    pub trip: Trip,
}

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

#[derive(InputObject)]
pub struct UpdateTripRidesInput {
    pub trip_id: ModelId<TripId>,
    pub ride_ids: Vec<ModelId<RideId>>,
}

#[derive(SimpleObject)]
pub struct TripRidesOutput {
    pub trip: Option<Trip>,
}

#[derive(SimpleObject)]
pub struct UpdateTripOutput {
    pub trip: Option<Trip>,
}

#[derive(InputObject)]
pub struct UpdateTripMediaInput {
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
    async fn create_trip(
        &self,
        ctx: &Context<'_>,
        input: CreateTripInput,
    ) -> Result<CreateTripOutput, Error> {
        // Get required context data
        let SchemaData {
            repos:
                Repos {
                    trip_repo,
                    ride_repo,
                    ..
                },
            ..
        } = ctx.data()?;
        let RequestData { login } = ctx.data()?;

        // Ensure user is logged in
        let login = login
            .as_ref()
            .ok_or_else(|| Error::new("Authentication required"))?;

        // Get all rides to validate ownership and determine year
        let rides = ride_repo
            .filter_models(RideFilter::Ids(
                input.ride_ids.iter().map(|id| id.0).collect(),
            ))
            .await?;

        // Verify all rides belong to user
        for ride in &rides {
            if ride.user_id != login.session.user_id {
                return Err(Error::new("Not authorized to use this ride"));
            }
        }

        // Get earliest ride to determine year
        let first_ride = rides
            .iter()
            .min_by_key(|ride| ride.started_at)
            .ok_or_else(|| Error::new("At least one ride is required"))?;

        let trip = TripModel {
            id: TripId::new(),
            created_at: Utc::now(),
            user_id: login.session.user_id,
            name: input.name.clone(),
            slug: generate_slug(&input.name),
            year: first_ride.started_at.year(),
            description: input.description,
            notes: Vec::new(),
            ride_ids: input.ride_ids.into_iter().map(|id| id.0).collect(),
            media_ids: Vec::new(),
        };

        // Save the new trip
        trip_repo.put(trip.clone()).await?;

        Ok(CreateTripOutput { trip: Trip(trip) })
    }

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

    async fn update_trip_rides(
        &self,
        ctx: &Context<'_>,
        input: UpdateTripRidesInput,
    ) -> Result<TripRidesOutput, Error> {
        // Get required context data
        let SchemaData {
            repos:
                Repos {
                    trip_repo,
                    ride_repo,
                    ..
                },
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

        // Get all rides to validate ownership
        let rides = ride_repo
            .filter_models(RideFilter::Ids(
                input.ride_ids.iter().map(|id| id.0).collect(),
            ))
            .await?;

        // Verify all rides belong to user
        for ride in &rides {
            if ride.user_id != login.session.user_id {
                return Err(Error::new("Not authorized to use this ride"));
            }
        }

        // Update ride IDs
        trip.ride_ids = input.ride_ids.into_iter().map(|id| id.0).collect();

        // Save changes
        trip_repo.put(trip.clone()).await?;

        Ok(TripRidesOutput {
            trip: Some(Trip(trip)),
        })
    }

    async fn update_trip_media(
        &self,
        ctx: &Context<'_>,
        input: UpdateTripMediaInput,
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

        // Update media IDs
        trip.media_ids = input.media_ids.into_iter().map(|id| id.0).collect();
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

    async fn initiate_rwgps_history_sync(&self, ctx: &Context<'_>) -> Result<Viewer, Error> {
        // Get required context data
        let SchemaData {
            repos: Repos { user_repo, .. },
            job_storage,
            ..
        } = ctx.data()?;
        let RequestData { login } = ctx.data()?;

        // Ensure user is logged in
        let login = login
            .as_ref()
            .ok_or_else(|| Error::new("Authentication required"))?
            .clone();

        // Get the user
        let user = user_repo.get(login.session.user_id).await?;

        // Check if user has RWGPS connection
        let connection = user
            .rwgps_connection
            .ok_or_else(|| Error::new("No RWGPS connection found"))?;

        // Enqueue the sync job
        job_storage
            .push(Job::from(RwgpsJob::SyncHistory {
                connection: connection.clone(),
            }))
            .await
            .map_err(|e| Error::new(format!("Failed to enqueue job: {}", e)))?;

        Ok(Viewer(login))
    }
}
