use howitt_client_types::RedisClient;

use crate::{
    models::{
        point::TemporalElevationPoint,
        ride::{RideId, RidePoints},
    },
    repos::RidePointsRepo,
    services::simplify_points::simplify_points,
};

use super::{cache::CacheFetcher, PointsFetcherParams};

pub struct SimplifiedRidePointsFetcher<Redis: RedisClient> {
    pub ride_points_repo: RidePointsRepo,
    pub cache_fetcher: CacheFetcher<Redis>,
}

impl<Redis: RedisClient> SimplifiedRidePointsFetcher<Redis> {
    pub fn new(ride_points_repo: RidePointsRepo, redis_client: Redis) -> Self {
        Self {
            ride_points_repo,
            cache_fetcher: CacheFetcher::new(redis_client),
        }
    }

    fn key(id: RideId, params: &PointsFetcherParams) -> String {
        [
            id.to_string(),
            "POINTS".to_string(),
            params.target.to_string(),
        ]
        .join("#")
    }

    pub async fn fetch(
        &self,
        id: RideId,
        params: PointsFetcherParams,
    ) -> Result<Vec<TemporalElevationPoint>, anyhow::Error> {
        let key = Self::key(id, &params);

        self.cache_fetcher
            .fetch_or_insert_with(&key, || async {
                let RidePoints { points, .. } = self.ride_points_repo.get(id).await?;
                Ok(
                    tokio::task::spawn_blocking(move || simplify_points(&points, params.target))
                        .await?,
                )
            })
            .await
    }
}
