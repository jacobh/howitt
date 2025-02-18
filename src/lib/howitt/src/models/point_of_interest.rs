use serde::{Deserialize, Serialize};

use super::{Model, ModelName, ModelUuid};

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
    pub name: String,
    pub point: geo::Point<f64>,
    pub point_of_interest_type: PointOfInterestType,
}

impl Model for PointOfInterest {
    type Id = PointOfInterestId;
    type Filter = ();

    fn id(&self) -> PointOfInterestId {
        PointOfInterestId::from(self.id)
    }
}
