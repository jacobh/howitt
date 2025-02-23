use async_graphql::{Context, Enum, Object};
use chrono::{DateTime, Utc};
use howitt::{models::media::MediaFilter, repos::Repos};

use crate::graphql::context::SchemaData;

use super::{media::Media, point_of_interest::PointOfInterest, user::UserProfile};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::point_of_interest_visit::POICondition")]
pub enum POICondition {
    AllGood,
    Issue,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::point_of_interest_visit::VisitConfirmation")]
pub enum VisitConfirmation {
    Pending,
    Confirmed,
    Rejected,
}

pub struct PointOfInterestVisit(pub howitt::models::point_of_interest_visit::PointOfInterestVisit);

#[Object]
impl PointOfInterestVisit {
    async fn visited_at(&self) -> DateTime<Utc> {
        self.0.visited_at
    }

    async fn condition(&self) -> Option<POICondition> {
        self.0.condition.clone().map(POICondition::from)
    }

    async fn confirmation(&self) -> VisitConfirmation {
        VisitConfirmation::from(self.0.confirmation.clone())
    }

    async fn comment(&self) -> Option<&str> {
        self.0.comment.as_deref()
    }

    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserProfile, async_graphql::Error> {
        let SchemaData { user_loader, .. } = ctx.data()?;

        let user = user_loader
            .load_one(self.0.user_id)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?;

        Ok(UserProfile(user))
    }

    async fn point_of_interest<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<PointOfInterest, async_graphql::Error> {
        let SchemaData {
            repos: Repos {
                point_of_interest_repo,
                ..
            },
            ..
        } = ctx.data()?;

        let poi = point_of_interest_repo
            .get(self.0.point_of_interest_id)
            .await?;

        Ok(PointOfInterest(poi))
    }

    async fn media<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Media>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { media_repo, .. },
            ..
        } = ctx.data()?;

        let media = media_repo
            .filter_models(MediaFilter::Ids(self.0.media_ids.clone()))
            .await?;

        Ok(media.into_iter().map(Media).collect())
    }
}
