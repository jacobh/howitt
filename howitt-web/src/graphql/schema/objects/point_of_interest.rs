use async_graphql::{Enum, Object};
use howitt::models::{point_of_interest::PointOfInterestId, Model};

use crate::graphql::schema::ModelId;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::point_of_interest::PointOfInterestType")]
pub enum PointOfInterestType {
    RailwayStation,
    Hut,
    Locality,
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
    async fn point(&self) -> Vec<f64> {
        vec![self.0.point.x(), self.0.point.y()]
    }
    async fn point_of_interest_type(&self) -> PointOfInterestType {
        PointOfInterestType::from(self.0.point_of_interest_type.clone())
    }
}
