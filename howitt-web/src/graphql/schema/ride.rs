use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use howitt::{
    models::{point::Point, ride::RideId},
    services::{fetchers::PointsFetcherParams, simplify_points::SimplifyTarget},
};
use itertools::Itertools;

use crate::graphql::context::SchemaData;

use super::ModelId;

pub struct Ride(pub howitt::models::ride::Ride);

#[Object]
impl Ride {
    async fn id(&self) -> ModelId<RideId> {
        ModelId::from(self.0.id)
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn distance(&self) -> f64 {
        self.0.distance
    }
    async fn started_at(&self) -> DateTime<Utc> {
        self.0.started_at
    }
    async fn finished_at(&self) -> DateTime<Utc> {
        self.0.finished_at
    }
    async fn points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        points_per_km: usize,
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        let SchemaData {
            simplified_ride_points_fetcher,
            ..
        } = ctx.data()?;
        let ride_points = simplified_ride_points_fetcher
            .fetch(
                self.0.id,
                PointsFetcherParams {
                    target: SimplifyTarget::PointPerKm(points_per_km),
                },
            )
            .await?;

        Ok(ride_points
            .into_iter()
            .map(|point| point.into_x_y_vec())
            .collect_vec())
    }
    async fn points_json<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        points_per_km: usize,
    ) -> Result<String, async_graphql::Error> {
        let points = self.points(ctx, points_per_km).await?;

        Ok(serde_json::to_string(&points)?)
    }
}
