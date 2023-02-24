use anyhow::anyhow;
use derive_more::From;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::models::{external_ref::ExternalRef, point::ElevationPoint};

use super::IndexItem;

crate::model_id!(RouteId, "ROUTE");

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: ulid::Ulid,
    pub name: String,
    pub distance: f64,
    pub external_ref: Option<ExternalRef>,
}

impl IndexItem for Route {
    type Id = RouteId;

    fn model_id(&self) -> Self::Id {
        RouteId::from(self.id)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoutePointChunk {
    pub route_id: ulid::Ulid,
    pub idx: usize,
    pub points: Vec<ElevationPoint>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RouteModel {
    pub route: Route,
    pub point_chunks: Vec<RoutePointChunk>,
}
impl RouteModel {
    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.point_chunks
            .iter()
            .flat_map(|chunk| chunk.points.iter())
            .map(|point| point.point.clone())
    }
}

impl crate::models::Model for RouteModel {
    type Id = RouteId;
    type IndexItem = Route;
    type OtherItem = RouteItem;

    fn id(&self) -> RouteId {
        RouteId::from(self.route.id)
    }

    fn into_parts(self) -> (Self::IndexItem, Vec<Self::OtherItem>) {
        (
            self.route,
            self.point_chunks.into_iter().map(RouteItem::from).collect(),
        )
    }

    fn from_parts(
        route: Self::IndexItem,
        other: Vec<Self::OtherItem>,
    ) -> Result<Self, anyhow::Error> {
        Ok(RouteModel {
            route,
            point_chunks: other
                .into_iter()
                .filter_map(RouteItem::into_point_chunk)
                .collect(),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, From)]
#[serde(tag = "item")]
pub enum RouteItem {
    PointChunk(RoutePointChunk),
}
impl RouteItem {
    fn into_point_chunk(self) -> Option<RoutePointChunk> {
        match self {
            RouteItem::PointChunk(chunk) => Some(chunk),
            _ => None,
        }
    }
}
impl crate::models::OtherItem for RouteItem {
    type Id = RouteId;

    fn item_name(&self) -> String {
        match self {
            RouteItem::PointChunk(_) => "POINT_CHUNK".to_string(),
        }
    }

    fn model_id(&self) -> RouteId {
        match self {
            RouteItem::PointChunk(chunk) => RouteId::from(chunk.route_id),
        }
    }

    fn item_id(&self) -> String {
        match self {
            RouteItem::PointChunk(chunk) => chunk.idx.to_string(),
        }
    }
}
