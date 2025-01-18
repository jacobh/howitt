use howitt_client_types::RedisClient;

use crate::{
    models::{
        point::TemporalElevationPoint,
        ride::{RideId, RidePoints},
    },
    repos::RidePointsRepo,
    services::simplify_points::{simplify_points, SimplifyTarget},
};

#[derive(Debug)]
pub struct PointsFetcherParams {
    pub target: SimplifyTarget,
}

pub struct SimplifiedRidePointsFetcher<Redis: RedisClient> {
    pub ride_points_repo: RidePointsRepo,
    pub redis_client: Redis,
}

impl<Redis: RedisClient> SimplifiedRidePointsFetcher<Redis> {
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

        let value = self.redis_client.get_bytes(&key).await?;

        if let Some(value) = value {
            return Ok(bincode::deserialize(&value)?);
        }

        let RidePoints { points, .. } = self.ride_points_repo.get(id).await?;

        let simplified = simplify_points(&points, params.target);

        let serialized = bincode::serialize(&simplified)?;

        self.redis_client.put_bytes(&key, serialized).await?;

        Ok(simplified)
    }
}
