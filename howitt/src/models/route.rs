use std::collections::HashSet;

use derive_more::From;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::{
    models::{external_ref::ExternalRef, point::ElevationPoint},
    services::summarize_segment::{summarize_segment, SummarizeError},
};

use super::{
    external_ref::ExternallySourced, point::PointChunk, route_description::RouteDescription,
    segment_summary::SegmentSummary, IndexItem,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum RouteTag {
    BackcountrySegment,
}

crate::model_id!(RouteId, "ROUTE");

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: ulid::Ulid,
    pub name: String,
    pub distance: f64,
    pub description: Option<RouteDescription>,
    pub external_ref: Option<ExternalRef>,
    #[serde(default)]
    pub tags: HashSet<RouteTag>,
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

#[derive(Debug, Clone)]
pub struct RouteModel {
    pub route: Route,
    pub point_chunks: Vec<PointChunk<RouteId, ElevationPoint>>,
    summary: OnceCell<Result<SegmentSummary<ElevationPoint>, SummarizeError>>,
}
impl RouteModel {
    pub fn new(route: Route, point_chunks: Vec<PointChunk<RouteId, ElevationPoint>>) -> RouteModel {
        RouteModel {
            route,
            point_chunks,
            summary: OnceCell::new(),
        }
    }

    pub fn iter_elevation_points(&self) -> impl Iterator<Item = &ElevationPoint> + '_ {
        PointChunk::iter_points(&self.point_chunks)
    }

    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.iter_elevation_points().map(|point| point.point)
    }

    pub fn segment_summary(&self) -> Result<&SegmentSummary<ElevationPoint>, &SummarizeError> {
        self.summary
            .get_or_init(|| {
                summarize_segment(
                    self.iter_elevation_points()
                        .cloned()
                        .collect_vec()
                        .as_slice(),
                )
            })
            .as_ref()
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
