use howitt_client_types::CacheStore;

use crate::{
    ext::rayon::rayon_spawn_blocking,
    models::{
        point::TemporalElevationPoint,
        ride::{RideId, RidePoints},
    },
    repos::RidePointsRepo,
    services::simplify_points::{simplify_points_v2, DetailLevel},
};

use super::cache::CacheFetcher;

pub struct SimplifiedRidePointsFetcher<Cache: CacheStore> {
    pub ride_points_repo: RidePointsRepo,
    pub cache_fetcher: CacheFetcher<Cache>,
}

impl<Cache: CacheStore> SimplifiedRidePointsFetcher<Cache> {
    pub fn new(ride_points_repo: RidePointsRepo, cache: Cache) -> Self {
        Self {
            ride_points_repo,
            cache_fetcher: CacheFetcher::new(cache),
        }
    }

    fn key(id: RideId, detail_level: &DetailLevel) -> String {
        [
            id.to_string(),
            "POINTS".to_string(),
            detail_level.to_string(),
        ]
        .join("#")
    }

    pub async fn fetch(
        &self,
        id: RideId,
        detail_level: DetailLevel,
    ) -> Result<Vec<TemporalElevationPoint>, anyhow::Error> {
        let key = Self::key(id, &detail_level);

        self.cache_fetcher
            .fetch_or_insert_with(&key, || async {
                let RidePoints { points, .. } = self.ride_points_repo.get(id).await?;

                tracing::info!(ride_id = ?id, points_count = points.len(), "starting points simplification");

                let points =
                    rayon_spawn_blocking(move || simplify_points_v2(points, detail_level)).await;

                tracing::info!(ride_id = ?id, points_count = points.len(), "completed points simplification");

                Ok(points)
            })
            .await
    }
}
