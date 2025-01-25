use howitt_client_types::RedisClient;

use crate::{
    models::{
        point::ElevationPoint,
        route::{RouteId, RouteModel},
    },
    repos::RouteModelRepo,
    services::simplify_points::simplify_points,
};

use super::PointsFetcherParams;

pub struct SimplifiedRoutePointsFetcher<Redis: RedisClient> {
    pub route_repo: RouteModelRepo,
    pub redis_client: Redis,
}

impl<Redis: RedisClient> SimplifiedRoutePointsFetcher<Redis> {
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

        let value = self.redis_client.get_bytes(&key).await?;

        if let Some(value) = value {
            return Ok(bincode::deserialize(&value)?);
        }

        let RouteModel { points, .. } = self.route_repo.get(id).await?;

        let simplified =
            tokio::task::spawn_blocking(move || simplify_points(&points, params.target)).await?;

        let serialized = bincode::serialize(&simplified)?;

        self.redis_client.set_bytes(&key, serialized.into()).await?;

        Ok(simplified)
    }
}
