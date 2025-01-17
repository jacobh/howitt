use chrono::{DateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::models::{external_ref::ExternalRef, point::TemporalElevationPoint};

use super::{
    external_ref::ExternallySourced, filters::TemporalFilter, user::UserId, IndexModel, ModelName,
    ModelUlid,
};

pub type RideId = ModelUlid<{ ModelName::Ride }>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Ride {
    pub id: RideId,
    pub name: String,
    pub distance: f64,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub external_ref: Option<ExternalRef>,
}

impl IndexModel for Ride {
    type Id = RideId;
    type Filter = RideFilter;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl ExternallySourced for Ride {
    fn external_ref(&self) -> Option<&ExternalRef> {
        self.external_ref.as_ref()
    }
}

#[derive(Debug, Clone)]
pub enum RideFilter {
    All,
    User {
        user_id: UserId,
        started_at: Option<TemporalFilter>,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RidePoints {
    pub id: RideId,
    pub points: Vec<TemporalElevationPoint>,
}

impl IndexModel for RidePoints {
    type Id = RideId;
    type Filter = ();

    fn id(&self) -> RideId {
        RideId::from(self.id)
    }
}
