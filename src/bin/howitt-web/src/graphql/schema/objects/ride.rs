use async_graphql::{Context, Enum, Object};
use chrono::{DateTime, Utc};
use howitt::{
    models::{
        media::MediaFilter,
        point::{
            progress::{DistanceProgress, Progress},
            Point,
        },
        ride::RideId,
    },
    repos::Repos,
    services::simplify_points::DetailLevel,
};
use itertools::Itertools;

use crate::graphql::context::SchemaData;

use crate::graphql::schema::{user::UserProfile, IsoDate, ModelId};

use super::media::Media;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PointsDetail {
    Low,
    Medium,
    High,
}

impl From<PointsDetail> for DetailLevel {
    fn from(detail: PointsDetail) -> Self {
        match detail {
            PointsDetail::Low => DetailLevel::Low,
            PointsDetail::Medium => DetailLevel::Medium,
            PointsDetail::High => DetailLevel::High,
        }
    }
}

#[derive(Clone)]
pub struct Ride(pub howitt::models::ride::Ride);

#[Object]
impl Ride {
    async fn id(&self) -> ModelId<RideId> {
        ModelId::from(self.0.id)
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn date(&self) -> IsoDate {
        IsoDate(
            self.0
                .started_at
                .with_timezone(&chrono_tz::Australia::Melbourne)
                .date_naive(),
        )
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
        detail_level: PointsDetail,
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        let SchemaData {
            simplified_ride_points_fetcher,
            ..
        } = ctx.data()?;
        let ride_points = simplified_ride_points_fetcher
            .fetch(self.0.id, DetailLevel::from(detail_level))
            .await?;

        Ok(ride_points
            .into_iter()
            .map(|point| point.into_x_y_vec())
            .collect_vec())
    }
    async fn points_json<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        detail_level: PointsDetail,
    ) -> Result<String, async_graphql::Error> {
        let points = self.points(ctx, detail_level).await?;

        Ok(serde_json::to_string(&points)?)
    }
    pub async fn elevation_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData {
            simplified_ride_points_fetcher,
            ..
        } = ctx.data()?;

        let ride_points = simplified_ride_points_fetcher
            .fetch(self.0.id, DetailLevel::High)
            .await?;

        Ok(ride_points
            .into_iter()
            .map(|point| point.elevation)
            .collect())
    }
    pub async fn distance_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData {
            simplified_ride_points_fetcher,
            ..
        } = ctx.data()?;

        let ride_points = simplified_ride_points_fetcher
            .fetch(self.0.id, DetailLevel::High)
            .await?;

        let progress = DistanceProgress::from_points(ride_points);

        Ok(progress.into_iter().map(|p| p.distance_m).collect())
    }
    pub async fn elevation_points_json<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<String, async_graphql::Error> {
        Ok(serde_json::to_string(&self.elevation_points(ctx).await?)?)
    }
    pub async fn distance_points_json<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<String, async_graphql::Error> {
        Ok(serde_json::to_string(&self.distance_points(ctx).await?)?)
    }
    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserProfile, async_graphql::Error> {
        let SchemaData { user_loader, .. } = ctx.data()?;

        let user = user_loader
            .load_one(self.0.user_id)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?;

        Ok(UserProfile(user))
    }
    pub async fn media<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Media>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { media_repo, .. },
            ..
        } = ctx.data()?;

        let media = media_repo
            .filter_models(MediaFilter::ForRide(self.0.id))
            .await?;

        Ok(media.into_iter().map(Media).collect())
    }
    pub async fn content_at(&self) -> DateTime<Utc> {
        self.0.started_at.clone()
    }
}
