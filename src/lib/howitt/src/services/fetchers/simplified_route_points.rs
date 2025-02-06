use howitt_client_types::RedisClient;

use crate::{
    models::{
        point::ElevationPoint,
        route::{RouteId, RoutePoints},
    },
    repos::RoutePointsRepo,
    services::simplify_points::simplify_points,
};

use super::{cache::CacheFetcher, PointsFetcherParams};

pub struct SimplifiedRoutePointsFetcher<Redis: RedisClient> {
    pub route_points_repo: RoutePointsRepo,
    pub cache_fetcher: CacheFetcher<Redis>,
}

impl<Redis: RedisClient> SimplifiedRoutePointsFetcher<Redis> {
    pub fn new(route_points_repo: RoutePointsRepo, redis_client: Redis) -> Self {
        Self {
            route_points_repo,
            cache_fetcher: CacheFetcher::new(redis_client),
        }
    }

    fn key(id: RouteId, params: &PointsFetcherParams) -> String {
        [
            id.to_string(),
            "POINTS".to_string(),
            params.target.to_string(),
        ]
        .join("#")
    }

    pub async fn fetch(
        &self,
        id: RouteId,
        params: PointsFetcherParams,
    ) -> Result<Vec<ElevationPoint>, anyhow::Error> {
        let key = Self::key(id, &params);

        self.cache_fetcher
            .fetch_or_insert_with(&key, || async {
                let RoutePoints { points, .. } = self.route_points_repo.get(id).await?;
                Ok(
                    tokio::task::spawn_blocking(move || simplify_points(&points, params.target))
                        .await?,
                )
            })
            .await
    }
}
