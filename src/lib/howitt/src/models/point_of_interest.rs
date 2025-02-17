use serde::{Deserialize, Serialize};

use super::{Model, ModelName, ModelUuid};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PointOfInterestType {
    RailwayStation,
    Hut,
    Locality,
    Generic,
}

pub type PointOfInterestId = ModelUuid<{ ModelName::Checkpoint }>;

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
