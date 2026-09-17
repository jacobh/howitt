use howitt::{
    models::{
        external_ref::{ExternalId, ExternalRef, RwgpsId},
        point::TemporalElevationPoint,
        ride::{Ride, RideId},
        user::UserRwgpsConnection,
    },
    repos::RideRepo,
};
use rwgps_types::{client::AuthenticatedRwgpsClient, credentials::Credentials};
use tracing;

use super::persistence::{DynRwgpsSyncStore, RwgpsSyncPersistenceError};

#[derive(Debug, thiserror::Error)]
#[error("RWGPS trip has fewer than two usable temporal/elevation points")]
pub struct InsufficientUsableTripPoints {
    pub usable_points: usize,
}

fn usable_trip_points(
    track_points: Vec<rwgps_types::TrackPoint>,
) -> Result<Vec<TemporalElevationPoint>, InsufficientUsableTripPoints> {
    let points = track_points
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

    if points.len() < 2 {
        return Err(InsufficientUsableTripPoints {
            usable_points: points.len(),
        });
    }

    Ok(points)
}

pub struct SyncTripParams<RwgpsClient> {
    pub client: RwgpsClient,
    pub ride_repo: RideRepo,
    pub store: DynRwgpsSyncStore,
    pub rwgps_trip_id: usize,
    pub connection: UserRwgpsConnection,
}

pub async fn sync_trip<RwgpsClient: rwgps_types::client::RwgpsClient>(
    SyncTripParams {
        client,
        rwgps_trip_id,
        connection,
        ride_repo,
        store,
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

    anyhow::ensure!(
        existing_ride
            .as_ref()
            .is_none_or(|ride| ride.user_id == connection.user_id),
        "RWGPS trip belongs to another user"
    );

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
    let points = usable_trip_points(rwgps_trip.track_points)?;

    tracing::info!(
        total_points = points.len(),
        "Converted track points to temporal elevation points"
    );

    // Get the time bounds from the points
    let started_at = points
        .iter()
        .map(|point| point.datetime)
        .min()
        .expect("usable trips have at least two points");

    let finished_at = points
        .iter()
        .map(|point| point.datetime)
        .max()
        .expect("usable trips have at least two points");

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

            store
                .save_trip(existing_ride, points)
                .await
                .map_err(RwgpsSyncPersistenceError::from)?;
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
            store
                .save_trip(ride, points)
                .await
                .map_err(RwgpsSyncPersistenceError::from)?;
            tracing::info!(ride_id = %id, "Successfully created new ride");
        }
    }

    tracing::info!(rwgps_trip_id, "Trip sync completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, Utc};
    use rwgps_types::TrackPoint;

    fn usable_point(seconds: i64) -> TrackPoint {
        TrackPoint {
            lng: Some(115.8 + seconds as f64 / 1000.0),
            lat: Some(-31.9),
            elevation: Some(23.0),
            datetime: Some(Utc::now() + TimeDelta::seconds(seconds)),
            ..Default::default()
        }
    }

    #[test]
    fn zero_usable_points_are_rejected() {
        let error = super::usable_trip_points(vec![]).unwrap_err();
        assert_eq!(error.usable_points, 0);
    }

    #[test]
    fn one_usable_point_is_rejected_even_with_other_incomplete_points() {
        let error = super::usable_trip_points(vec![
            usable_point(0),
            TrackPoint {
                lng: Some(115.9),
                lat: Some(-32.0),
                elevation: Some(47.0),
                datetime: None,
                ..Default::default()
            },
        ])
        .unwrap_err();
        assert_eq!(error.usable_points, 1);
    }

    #[test]
    fn two_usable_points_are_accepted() {
        let points = super::usable_trip_points(vec![usable_point(0), usable_point(60)]).unwrap();
        assert_eq!(points.len(), 2);
    }
}
