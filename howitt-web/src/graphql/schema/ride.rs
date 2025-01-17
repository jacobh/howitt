use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use howitt::models::ride::RideId;

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
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        // let SchemaData {
        //     ride_points_repo, ..
        // } = ctx.data()?;
        // let ride_model = ride_repo.get(self.0.id).await?;

        Ok(vec![])
    }
}
