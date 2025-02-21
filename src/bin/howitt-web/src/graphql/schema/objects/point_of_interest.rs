use async_graphql::{Context, Enum, Object};
use howitt::{
    models::{media::MediaFilter, point_of_interest::PointOfInterestId, Model},
    repos::Repos,
};

use crate::graphql::{context::SchemaData, schema::ModelId};

use super::{media::Media, point_of_interest_visit::PointOfInterestVisit};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::point_of_interest::PointOfInterestType")]
pub enum PointOfInterestType {
    PublicTransportStop,
    Campsite,
    WaterSource,
    Hut,
    Generic,
}

pub struct PointOfInterest(pub howitt::models::point_of_interest::PointOfInterest);

#[Object]
impl PointOfInterest {
    async fn id<'a>(&'a self) -> ModelId<PointOfInterestId> {
        ModelId::from(self.0.id())
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn slug(&self) -> &str {
        &self.0.slug
    }

    async fn point(&self) -> Vec<f64> {
        vec![self.0.point.x(), self.0.point.y()]
    }

    async fn point_of_interest_type(&self) -> PointOfInterestType {
        PointOfInterestType::from(self.0.point_of_interest_type.clone())
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
            .filter_models(MediaFilter::ForPointOfInterest(self.0.id))
            .await?;

        Ok(media.into_iter().map(Media).collect())
    }

    pub async fn visits<'ctx>(
        &self,
        _ctx: &Context<'ctx>,
    ) -> Result<Vec<PointOfInterestVisit>, async_graphql::Error> {
        Ok(vec![])
    }
}
