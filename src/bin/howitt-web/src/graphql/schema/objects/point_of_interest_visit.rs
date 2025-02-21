use async_graphql::{Context, Enum, Object};
use chrono::{DateTime, Utc};
use howitt::{models::media::MediaFilter, repos::Repos};

use crate::graphql::context::SchemaData;

use super::{media::Media, point_of_interest::PointOfInterest, user::UserProfile};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::point_of_interest_visit::VisitStatus")]
pub enum VisitStatus {
    AllGood,
    Issue,
}

pub struct PointOfInterestVisit(pub howitt::models::point_of_interest_visit::PointOfInterestVisit);

#[Object]
impl PointOfInterestVisit {
    async fn visited_at(&self) -> DateTime<Utc> {
        self.0.visited_at
    }

    async fn status(&self) -> VisitStatus {
        VisitStatus::from(self.0.status.clone())
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
