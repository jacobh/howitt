use anyhow::anyhow;
use derive_more::From;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{external_ref::ExternalRef, point::ElevationPoint};

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

impl crate::model::Model for RouteModel {
    type Item = RouteItem;

    fn model_name() -> &'static str {
        "ROUTE"
    }

    fn id(&self) -> String {
        self.route.id.to_string()
    }

    fn into_items(self) -> impl Iterator<Item = Self::Item> {
        vec![RouteItem::from(self.route)]
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
impl crate::model::Item for RouteItem {
    fn item_name(&self) -> &'static str {
        match self {
            RouteItem::Route(_) => "META",
            RouteItem::PointChunk(_) => "POINT_CHUNK",
        }
    }

    fn model_id(&self) -> String {
        match self {
            RouteItem::Route(route) => route.id.to_string(),
            RouteItem::PointChunk(chunk) => chunk.route_id.to_string(),
        }
    }

    fn item_id(&self) -> Option<String> {
        match self {
            RouteItem::Route(_) => None,
            RouteItem::PointChunk(chunk) => Some(chunk.idx.to_string()),
        }
    }
}
