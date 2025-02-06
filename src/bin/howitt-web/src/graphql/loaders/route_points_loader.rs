use async_graphql::dataloader::Loader;
use howitt::models::route::{RouteId, RoutePoints, RoutePointsFilter};
use howitt::repos::RoutePointsRepo;
use std::{collections::HashMap, sync::Arc};

pub struct RoutePointsLoader {
    route_points_repo: RoutePointsRepo,
}

impl RoutePointsLoader {
    pub fn new(route_points_repo: RoutePointsRepo) -> Self {
        Self { route_points_repo }
    }
}

impl Loader<RouteId> for RoutePointsLoader {
    type Value = RoutePoints;
    type Error = Arc<anyhow::Error>;

    async fn load(&self, keys: &[RouteId]) -> Result<HashMap<RouteId, Self::Value>, Self::Error> {
        let route_points = self
            .route_points_repo
            .filter_models(RoutePointsFilter::Ids(keys.to_vec()))
            .await
            .map_err(|e| Arc::new(e.into()))?;

        Ok(route_points
            .into_iter()
            .map(|points| (points.id, points))
            .collect())
    }
}
