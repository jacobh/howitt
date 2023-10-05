use std::{
    cell::{Ref, RefCell},
    sync::{Arc, Mutex},
};

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::{
    models::{external_ref::ExternalRef, point::ElevationPoint},
    services::summarize_segment::summarize_segment,
};

use super::{
    external_ref::ExternallySourced,
    point::PointChunk,
    route_description::RouteDescription,
    segment_summary::{ElevationSummary, SegmentSummary},
    IndexItem,
};

crate::model_id!(RouteId, "ROUTE");

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: ulid::Ulid,
    pub name: String,
    pub distance: f64,
    pub description: Option<RouteDescription>,
    pub external_ref: Option<ExternalRef>,
}

impl IndexItem for Route {
    type Id = RouteId;

    fn model_id(&self) -> Self::Id {
        RouteId::from(self.id)
    }
}

impl ExternallySourced for Route {
    fn external_ref(&self) -> Option<&ExternalRef> {
        self.external_ref.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteModel {
    pub route: Route,
    pub point_chunks: Vec<PointChunk<RouteId, ElevationPoint>>,
    #[serde(default)]
    summary: Arc<Mutex<RefCell<Option<SegmentSummary>>>>,
}
impl RouteModel {
    pub fn new(route: Route, point_chunks: Vec<PointChunk<RouteId, ElevationPoint>>) -> RouteModel {
        RouteModel {
            route,
            point_chunks,
            summary: Default::default(),
        }
    }

    pub fn iter_elevation_points(&self) -> impl Iterator<Item = &ElevationPoint> + '_ {
        self.point_chunks
            .iter()
            .flat_map(|chunk| chunk.points.iter())
    }

    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.iter_elevation_points().map(|point| point.point)
    }

    fn summary(&self) -> Option<Ref<'_, SegmentSummary>> {
        let cell = self.summary.lock().unwrap();

        cell.replace_with(|value| match value {
            Some(summary) => Some(*summary),
            None => Some(summarize_segment(self.iter_elevation_points())),
        });

        Ref::filter_map(cell.borrow(), |summary| summary.as_ref()).ok()
    }

    pub fn elevation_summary(&self) -> Option<Ref<'_, ElevationSummary>> {
        self.summary()
            .and_then(|summary| Ref::filter_map(summary, |summary| summary.elevation.as_ref()).ok())
    }
}

impl PartialEq for RouteModel {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route && self.point_chunks == other.point_chunks
    }
}

impl crate::models::Model for RouteModel {
    type Id = RouteId;
    type IndexItem = Route;
    type OtherItem = RouteItem;

    fn id(&self) -> RouteId {
        RouteId::from(self.route.id)
    }

    fn as_index(&self) -> &Self::IndexItem {
        &self.route
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
        Ok(RouteModel::new(
            route,
            other
                .into_iter()
                .filter_map(RouteItem::into_point_chunk)
                .collect(),
        ))
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, From)]
#[serde(tag = "item")]
pub enum RouteItem {
    PointChunk(PointChunk<RouteId, ElevationPoint>),
}
impl RouteItem {
    fn into_point_chunk(self) -> Option<PointChunk<RouteId, ElevationPoint>> {
        match self {
            RouteItem::PointChunk(chunk) => Some(chunk),
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
            RouteItem::PointChunk(chunk) => chunk.model_id,
        }
    }

    fn item_id(&self) -> String {
        match self {
            RouteItem::PointChunk(chunk) => chunk.idx.to_string(),
        }
    }
}
