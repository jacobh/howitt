use howitt_client_types::CacheStore;

use crate::{
    ext::rayon::rayon_spawn_blocking,
    models::{
        point::ElevationPoint,
        route::{RouteId, RoutePoints},
    },
    repos::RoutePointsRepo,
    services::simplify_points::{simplify_points_v2, DetailLevel},
};

use super::cache::CacheFetcher;

pub struct SimplifiedRoutePointsFetcher<Cache: CacheStore> {
    pub route_points_repo: RoutePointsRepo,
    pub cache_fetcher: CacheFetcher<Cache>,
}

impl<Cache: CacheStore> SimplifiedRoutePointsFetcher<Cache> {
    pub fn new(route_points_repo: RoutePointsRepo, cache: Cache) -> Self {
        Self {
            route_points_repo,
            cache_fetcher: CacheFetcher::new(cache),
        }
    }

    fn key(id: RouteId, detail_level: &DetailLevel) -> String {
        [
            id.to_string(),
            "POINTS".to_string(),
            detail_level.to_string(),
        ]
        .join("#")
    }

    pub async fn fetch(
        &self,
        id: RouteId,
        detail_level: DetailLevel,
    ) -> Result<Vec<ElevationPoint>, anyhow::Error> {
        let key = Self::key(id, &detail_level);

        self.cache_fetcher
            .fetch_or_insert_with(&key, || async {
                let RoutePoints { points, .. } = self.route_points_repo.get(id).await?;

                tracing::info!(route_id = ?id, points_count = points.len(), "starting points simplification");

                let points =
                    rayon_spawn_blocking(move || simplify_points_v2(points, detail_level)).await;

                tracing::info!(route_id = ?id, points_count = points.len(), "completed points simplification");

                Ok(points)
            })
            .await
    }
}
