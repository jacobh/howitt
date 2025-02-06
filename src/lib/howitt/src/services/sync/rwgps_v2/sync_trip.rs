use howitt::{
    models::{
        external_ref::{ExternalId, ExternalRef, RwgpsId},
        point::TemporalElevationPoint,
        ride::{Ride, RideId},
        user::UserRwgpsConnection,
    },
    repos::{RidePointsRepo, RideRepo},
};
use rwgps_types::{client::AuthenticatedRwgpsClient, credentials::Credentials};
use tracing;

pub struct SyncTripParams<RwgpsClient> {
    pub client: RwgpsClient,
    pub ride_repo: RideRepo,
    pub ride_points_repo: RidePointsRepo,
    pub rwgps_trip_id: usize,
    pub connection: UserRwgpsConnection,
}

pub async fn sync_trip<RwgpsClient: rwgps_types::client::RwgpsClient>(
    SyncTripParams {
        client,
        rwgps_trip_id,
        connection,
        ride_repo,
        ride_points_repo,
    }: SyncTripParams<RwgpsClient>,
) -> Result<(), anyhow::Error> {
    tracing::info!(
        rwgps_trip_id,
        user_id = %connection.user_id,
        "Starting trip sync"
    );

    // Check for existing ride
    let existing_ride = ride_repo
        .find_model(howitt::models::ride::RideFilter::RwgpsId(rwgps_trip_id))
        .await?;

    tracing::info!(
        ride_exists = existing_ride.is_some(),
        "Checked for existing ride"
    );

    // Create authenticated client
    let client = client.with_credentials(Credentials::from_token(connection.access_token));

    // Fetch trip details from RWGPS
    tracing::info!("Fetching trip details from RWGPS");
    let rwgps_trip = client.trip(rwgps_trip_id).await?;

    // Convert track points to TemporalElevationPoints
    let points = rwgps_trip
        .track_points
        .into_iter()
        .filter_map(|track_point| {
            match (
                geo::Point::try_from(track_point.clone()),
                track_point.elevation,
                track_point.datetime,
            ) {
                (Ok(point), Some(elevation), Some(datetime)) => Some(TemporalElevationPoint {
                    point,
                    elevation,
                    datetime,
                }),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    tracing::info!(
        total_points = points.len(),
        "Converted track points to temporal elevation points"
    );

    // Get the time bounds from the points
    let started_at = points
        .iter()
        .map(|point| point.datetime)
        .min()
        .ok_or_else(|| anyhow::anyhow!("No points found in trip"))?;

    let finished_at = points
        .iter()
        .map(|point| point.datetime)
        .max()
        .ok_or_else(|| anyhow::anyhow!("No points found in trip"))?;

    match existing_ride {
        Some(mut existing_ride) => {
            tracing::info!(
                ride_id = %existing_ride.id,
                "Updating existing ride"
            );

            // Update the existing ride
            existing_ride.external_ref = Some(ExternalRef {
                id: ExternalId::Rwgps(RwgpsId::Trip(rwgps_trip_id)),
                sync_version: Some(2),
                updated_at: rwgps_trip.updated_at,
            });
            existing_ride.started_at = started_at;
            existing_ride.finished_at = finished_at;

            ride_repo.put(existing_ride.clone()).await?;

            ride_points_repo
                .put(howitt::models::ride::RidePoints {
                    id: existing_ride.id,
                    points,
                })
                .await?;
            tracing::info!("Successfully updated ride and points");
        }
        None => {
            tracing::info!("Creating new ride");
            // Create new ride
            let id = RideId::new();
            let ride = Ride {
                id,
                name: rwgps_trip.name,
                user_id: connection.user_id,
                distance: rwgps_trip.distance,
                started_at,
                finished_at,
                external_ref: Some(ExternalRef {
                    id: ExternalId::Rwgps(RwgpsId::Trip(rwgps_trip_id)),
                    sync_version: Some(2),
                    updated_at: rwgps_trip.updated_at,
                }),
            };

            // Save new ride and points
            ride_repo.put(ride).await?;
            ride_points_repo
                .put(howitt::models::ride::RidePoints { id, points })
                .await?;
            tracing::info!(ride_id = %id, "Successfully created new ride");
        }
    }

    tracing::info!(rwgps_trip_id, "Trip sync completed successfully");
    Ok(())
}
