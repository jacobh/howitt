use async_graphql::*;
use chrono::{DateTime, Datelike, Utc};
use howitt::jobs::rwgps::RwgpsJob;
use howitt::jobs::Job;
use howitt::models::media::MediaId;
use howitt::models::point_of_interest::{PointOfInterest as PoiModel, PointOfInterestId};
use howitt::models::ride::{RideFilter, RideId};
use howitt::models::trip::{Trip as TripModel, TripId, TripNote};
use howitt::repos::Repos;
use howitt::services::slug::generate_slug;

use crate::graphql::context::{RequestData, SchemaData};
use crate::graphql::schema::{point_of_interest::PointOfInterest, trip::Trip, ModelId};

use super::point_of_interest::PointOfInterestType;
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
    pub is_published: bool,
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

#[derive(InputObject)]
pub struct CreatePointOfInterestInput {
    pub name: String,
    pub point: Vec<f64>,
    pub point_of_interest_type: PointOfInterestType,
    pub description: Option<String>,
}

#[derive(SimpleObject)]
pub struct CreatePointOfInterestOutput {
    pub point_of_interest: PointOfInterest,
}

#[derive(InputObject)]
pub struct UpdatePointOfInterestInput {
    pub point_of_interest_id: ModelId<PointOfInterestId>,
    pub name: String,
    pub point: Vec<f64>,
    pub point_of_interest_type: PointOfInterestType,
    pub description: Option<String>,
}

#[derive(SimpleObject)]
pub struct UpdatePointOfInterestOutput {
    pub point_of_interest: Option<PointOfInterest>,
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
            is_published: false,
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
        trip.is_published = input.is_published;
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

    async fn create_point_of_interest(
        &self,
        ctx: &Context<'_>,
        input: CreatePointOfInterestInput,
    ) -> Result<CreatePointOfInterestOutput, Error> {
        let SchemaData {
            repos: Repos {
                point_of_interest_repo,
                ..
            },
            ..
        } = ctx.data()?;
        let RequestData { login } = ctx.data()?;

        let login = login
            .as_ref()
            .ok_or_else(|| Error::new("Authentication required"))?;

        let point = geo::Point::new(input.point[0], input.point[1]);

        let poi = PoiModel {
            id: PointOfInterestId::new(),
            user_id: login.session.user_id,
            name: input.name.clone(),
            slug: generate_slug(&input.name),
            point,
            point_of_interest_type: input.point_of_interest_type.into(),
            description: input.description,
        };

        point_of_interest_repo.put(poi.clone()).await?;

        Ok(CreatePointOfInterestOutput {
            point_of_interest: PointOfInterest(poi),
        })
    }

    async fn update_point_of_interest(
        &self,
        ctx: &Context<'_>,
        input: UpdatePointOfInterestInput,
    ) -> Result<UpdatePointOfInterestOutput, Error> {
        let SchemaData {
            repos: Repos {
                point_of_interest_repo,
                ..
            },
            ..
        } = ctx.data()?;
        let RequestData { login } = ctx.data()?;

        let login = login
            .as_ref()
            .ok_or_else(|| Error::new("Authentication required"))?;

        let mut poi = point_of_interest_repo
            .get(input.point_of_interest_id.0)
            .await?;

        if poi.user_id != login.session.user_id {
            return Err(Error::new(
                "Not authorized to update this point of interest",
            ));
        }

        let point = geo::Point::new(input.point[0], input.point[1]);

        poi.name = input.name;
        poi.point = point;
        poi.point_of_interest_type = input.point_of_interest_type.into();
        poi.description = input.description;

        point_of_interest_repo.put(poi.clone()).await?;

        Ok(UpdatePointOfInterestOutput {
            point_of_interest: Some(PointOfInterest(poi)),
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
