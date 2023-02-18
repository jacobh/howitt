use anyhow::anyhow;
use chrono::{DateTime, Utc};
use derive_more::From;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::models::{external_ref::ExternalRef, point::TemporalElevationPoint};

crate::model_id!(RideId, "RIDE");

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Ride {
    pub id: RideId,
    pub name: String,
    pub distance: f64,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub external_ref: Option<ExternalRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RidePointChunk {
    pub ride_id: RideId,
    pub idx: usize,
    pub points: Vec<TemporalElevationPoint>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RideModel {
    pub ride: Ride,
    pub point_chunks: Vec<RidePointChunk>,
}
impl RideModel {
    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.point_chunks
            .iter()
            .flat_map(|chunk| chunk.points.iter())
            .map(|point| point.point.clone())
    }
}

impl crate::models::Model for RideModel {
    type Id = RideId;
    type Item = RideItem;

    fn id(&self) -> RideId {
        self.ride.id
    }

    fn into_items(self) -> impl Iterator<Item = Self::Item> {
        [RideItem::from(self.ride)]
            .into_iter()
            .chain(self.point_chunks.into_iter().map(RideItem::from))
    }

    fn from_items(items: Vec<Self::Item>) -> Result<Self, anyhow::Error> {
        Ok(RideModel {
            ride: items
                .iter()
                .filter_map(RideItem::as_ride)
                .nth(0)
                .ok_or(anyhow!("couldnt find meta"))?
                .clone(),
            point_chunks: items
                .iter()
                .filter_map(RideItem::as_point_chunk)
                .cloned()
                .collect_vec(),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, From)]
#[serde(tag = "item")]
pub enum RideItem {
    Ride(Ride),
    PointChunk(RidePointChunk),
}
impl RideItem {
    fn as_ride(&self) -> Option<&Ride> {
        match self {
            RideItem::Ride(ride) => Some(ride),
            _ => None,
        }
    }
    fn as_point_chunk(&self) -> Option<&RidePointChunk> {
        match self {
            RideItem::PointChunk(chunk) => Some(chunk),
            _ => None,
        }
    }
}
impl crate::models::Item for RideItem {
    type Id = RideId;

    fn item_name(&self) -> Option<String> {
        match self {
            RideItem::Ride(_) => None,
            RideItem::PointChunk(_) => Some("POINT_CHUNK".to_string()),
        }
    }

    fn model_id(&self) -> RideId {
        match self {
            RideItem::Ride(ride) => RideId::from(ride.id),
            RideItem::PointChunk(chunk) => RideId::from(chunk.ride_id),
        }
    }

    fn item_id(&self) -> Option<String> {
        match self {
            RideItem::Ride(_) => None,
            RideItem::PointChunk(chunk) => Some(chunk.idx.to_string()),
        }
    }
}
