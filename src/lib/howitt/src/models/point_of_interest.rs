use serde::{Deserialize, Serialize};

use super::{user::UserId, Model, ModelName, ModelUuid};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PointOfInterestType {
    PublicTransportStop,
    Campsite,
    WaterSource,
    Hut,
    Generic,
}

pub type PointOfInterestId = ModelUuid<{ ModelName::PointOfInterest }>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInterest {
    pub id: PointOfInterestId,
    pub user_id: UserId,
    pub name: String,
    pub slug: String,
    pub point: geo::Point<f64>,
    pub point_of_interest_type: PointOfInterestType,
    pub description: Option<String>,
}

impl Model for PointOfInterest {
    type Id = PointOfInterestId;
    type Filter = ();

    fn id(&self) -> PointOfInterestId {
        PointOfInterestId::from(self.id)
    }
}
