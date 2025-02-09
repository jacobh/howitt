use anyhow::anyhow;
use derive_more::derive::Display;
use geo::{CoordsIter, LineString, SimplifyVw};
use howitt_client_types::RedisClient;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    ext::{futures::FuturesIteratorExt, iter::ResultIterExt},
    models::{
        point::progress::{DistanceProgress, Progress},
        ride::RideFilter,
        trip::TripId,
    },
    repos::{RidePointsRepo, RideRepo},
};

use super::cache::CacheFetcher;

#[derive(Debug, Display)]
#[display("EPSILON#{}", self.epsilon)]
pub struct ElevationPointsParams {
    epsilon: f64,
}

impl ElevationPointsParams {
    pub fn from_epsilon(epsilon: f64) -> Self {
        Self { epsilon }
    }
}

impl Default for ElevationPointsParams {
    fn default() -> Self {
        Self { epsilon: 50.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceElevation(pub f64, pub f64);

pub struct SimplifiedTripElevationPointsFetcher<Redis: RedisClient> {
    pub ride_repo: RideRepo,
    pub ride_points_repo: RidePointsRepo,
    pub cache_fetcher: CacheFetcher<Redis>,
}

impl<Redis: RedisClient> SimplifiedTripElevationPointsFetcher<Redis> {
    pub fn new(ride_repo: RideRepo, ride_points_repo: RidePointsRepo, redis_client: Redis) -> Self {
        Self {
            ride_repo,
            ride_points_repo,
            cache_fetcher: CacheFetcher::new(redis_client),
        }
    }

    fn key(id: TripId, params: &ElevationPointsParams) -> String {
        [
            id.to_string(),
            "TRIP_ELEVATION_POINTS".to_string(),
            params.to_string(),
        ]
        .join("#")
    }

    pub async fn fetch(
        &self,
        id: TripId,
        params: ElevationPointsParams,
    ) -> Result<Vec<DistanceElevation>, anyhow::Error> {
        let key = Self::key(id, &params);

        self.cache_fetcher
            .fetch_or_insert_with(&key, || async {
                // Get all rides for this trip
                let rides = self
                    .ride_repo
                    .filter_models(RideFilter::ForTrip(id))
                    .await?;

                // Fetch all points and chain them together
                tracing::info!("Fetching points for all rides...");
                let all_points = rides
                    .iter()
                    .map(|ride| self.ride_points_repo.get(ride.id))
                    .collect_futures_ordered()
                    .await
                    .into_iter()
                    .collect_result_vec()?
                    .into_iter()
                    .flat_map(|ride_points| ride_points.points)
                    .collect_vec();
                tracing::info!("Total points collected: {}", all_points.len());

                // Generate distance progress for all points
                tracing::info!("Calculating distance progress...");
                let progress: Vec<_> = DistanceProgress::from_points(all_points).collect();
                tracing::info!(
                    "Progress calculations complete for {} points",
                    progress.len()
                );

                // Find the total distance for normalization
                let total_distance = progress
                    .last()
                    .map(|p| p.distance_m)
                    .ok_or(anyhow!("No points found"))?;

                let distance_elevation_coords: LineString<f64> = progress
                    .into_iter()
                    .map(|p| {
                        geo::coord! {
                            x: (p.distance_m / total_distance) * 100_000.0,
                            y: p.point.elevation
                        }
                    })
                    .collect();

                let simplified =
                    SimplifyVw::simplify_vw(&distance_elevation_coords, &params.epsilon);

                tracing::info!("Simplified to {} points", simplified.coords_count());

                let elevation_distances = simplified
                    .into_iter()
                    .map(|coord| {
                        // Denormalize the distance back to meters
                        let real_distance = (coord.x / 100_000.0) * total_distance;
                        DistanceElevation(real_distance, coord.y)
                    })
                    .collect_vec();

                Ok(elevation_distances)
            })
            .await
    }
}
