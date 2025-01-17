use async_graphql::{Context, Object};
use chrono::{Duration, Utc};
use howitt::models::{ride::RideFilter, user::UserId};
use itertools::Itertools;

use crate::graphql::context::SchemaData;

use super::{ride::Ride, trip::Trip, ModelId};

pub struct UserProfile(pub howitt::models::user::User);

#[Object]
impl UserProfile {
    async fn id(&self) -> ModelId<UserId> {
        ModelId::from(self.0.id)
    }
    async fn username(&self) -> &str {
        &self.0.username
    }
    async fn recent_rides<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData { ride_repo, .. } = ctx.data()?;

        let rides = ride_repo
            .filter_models(RideFilter::User {
                user_id: self.0.id,
                started_at: Some(howitt::models::filters::TemporalFilter::After {
                    after: Utc::now() - Duration::days(365),
                    first: None,
                }),
            })
            .await?;

        let rides = rides.into_iter().map(Ride).collect_vec();

        Ok(rides)
    }
    async fn trips(&self) -> Result<Vec<Trip>, async_graphql::Error> {
        Ok(vec![])
    }
}
