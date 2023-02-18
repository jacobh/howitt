use anyhow::anyhow;
use derive_more::From;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::models::{external_ref::ExternalRef, point::ElevationPoint};

crate::model_id!(RouteId, "ROUTE");

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: ulid::Ulid,
    pub name: String,
    pub distance: f64,
    pub external_ref: Option<ExternalRef>,
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
    type Item = RouteItem;

    fn id(&self) -> RouteId {
        RouteId::from(self.route.id)
    }

    fn into_items(self) -> impl Iterator<Item = Self::Item> {
        [RouteItem::from(self.route)]
            .into_iter()
            .chain(self.point_chunks.into_iter().map(RouteItem::from))
    }

    fn from_items(items: Vec<Self::Item>) -> Result<Self, anyhow::Error> {
        Ok(RouteModel {
            route: items
                .iter()
                .filter_map(RouteItem::as_route)
                .nth(0)
                .ok_or(anyhow!("couldnt find meta"))?
                .clone(),
            point_chunks: items
                .iter()
                .filter_map(RouteItem::as_point_chunk)
                .cloned()
                .collect_vec(),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, From)]
#[serde(tag = "item")]
pub enum RouteItem {
    Route(Route),
    PointChunk(RoutePointChunk),
}
impl RouteItem {
    fn as_route(&self) -> Option<&Route> {
        match self {
            RouteItem::Route(route) => Some(route),
            _ => None,
        }
    }
    fn as_point_chunk(&self) -> Option<&RoutePointChunk> {
        match self {
            RouteItem::PointChunk(chunk) => Some(chunk),
            _ => None,
        }
    }
}
impl crate::models::Item for RouteItem {
    type Id = RouteId;

    fn item_name(&self) -> Option<String> {
        match self {
            RouteItem::Route(_) => None,
            RouteItem::PointChunk(_) => Some("POINT_CHUNK".to_string()),
        }
    }

    fn model_id(&self) -> RouteId {
        match self {
            RouteItem::Route(route) => RouteId::from(route.id),
            RouteItem::PointChunk(chunk) => RouteId::from(chunk.route_id),
        }
    }

    fn item_id(&self) -> Option<String> {
        match self {
            RouteItem::Route(_) => None,
            RouteItem::PointChunk(chunk) => Some(chunk.idx.to_string()),
        }
    }
}
