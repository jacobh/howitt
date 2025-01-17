use chrono::{DateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::models::{external_ref::ExternalRef, point::TemporalElevationPoint};

use super::{
    external_ref::ExternallySourced, filters::TemporalFilter, point::PointChunk, user::UserId,
    IndexModel, ModelName, ModelUlid,
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
    type Filter = ();

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
pub struct RideModel {
    pub ride: Ride,
    pub point_chunks: Vec<PointChunk<RideId, TemporalElevationPoint>>,
}
impl RideModel {
    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.point_chunks
            .iter()
            .flat_map(|chunk| chunk.points.iter())
            .map(|point| point.point)
    }
}

impl crate::models::Model for RideModel {
    type Id = RideId;
    type IndexItem = Ride;
    type Filter = RideFilter;

    fn id(&self) -> RideId {
        self.ride.id
    }

    fn as_index(&self) -> &Self::IndexItem {
        &self.ride
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, From)]
#[serde(tag = "item")]
pub enum RideItem {
    PointChunk(PointChunk<RideId, TemporalElevationPoint>),
}

impl crate::models::OtherItem for RideItem {
    type Id = RideId;

    fn item_name(&self) -> String {
        match self {
            RideItem::PointChunk(_) => "POINT_CHUNK".to_string(),
        }
    }

    fn model_id(&self) -> RideId {
        match self {
            RideItem::PointChunk(chunk) => chunk.model_id,
        }
    }

    fn item_id(&self) -> String {
        match self {
            RideItem::PointChunk(chunk) => chunk.idx.to_string(),
        }
    }
}
