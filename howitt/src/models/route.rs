use std::collections::HashSet;

use derive_more::From;
use either::Either;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::{
    models::{external_ref::ExternalRef, point::ElevationPoint},
    services::{
        nearby::{nearby_routes, NearbyRoute},
        summarize_segment::summarize_segment,
    },
};

use super::{
    external_ref::ExternallySourced,
    photo::Photo,
    point::{generate_point_deltas, ElevationPointDelta, PointChunk, PointDelta},
    route_description::RouteDescription,
    segment_summary::SegmentSummary,
    tag::Tag,
    terminus::{Termini, TerminusEnd},
    IndexItem, ModelUlid,
};

pub type RouteId = ModelUlid<"ROUTE">;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    #[serde(with = "either::serde_untagged")]
    pub id: Either<ulid::Ulid, RouteId>,
    pub name: String,
    pub distance: f64,
    pub sample_points: Option<Vec<ElevationPoint>>,
    pub description: Option<RouteDescription>,
    pub external_ref: Option<ExternalRef>,
    #[serde(default)]
    pub tags: HashSet<Tag>,
}

impl Route {
    pub fn id(&self) -> RouteId {
        match self.id {
            Either::Left(ulid) => RouteId::from(ulid),
            Either::Right(route_id) => route_id,
        }
    }
    pub fn published_at(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.tags.iter().find_map(|tag| match tag {
            Tag::Published { published_at } => Some(published_at),
            _ => None,
        })
    }

    pub fn termini(&self) -> Option<Termini<&ElevationPoint>> {
        self.sample_points.as_ref().and_then(|sample_points| {
            match (sample_points.first(), sample_points.last()) {
                (Some(p1), Some(p2)) => Some(Termini::new(p1, p2)),
                _ => None,
            }
        })
    }

    pub fn nearby_routes<'a, 'b>(
        &'a self,
        routes: &'b [Route],
    ) -> Vec<NearbyRoute<'a, 'b, ElevationPoint>> {
        nearby_routes(self, routes)
    }

    pub fn routes_near_terminus<'a, 'b>(
        &'a self,
        routes: &'b [Route],
        end: TerminusEnd,
    ) -> Vec<NearbyRoute<'a, 'b, ElevationPoint>> {
        if let Some(termini) = self.termini() {
            self.nearby_routes(routes)
                .into_iter()
                .filter(|(point, _, _, _)| termini.closest_terminus(*point).end == end)
                .collect_vec()
        } else {
            vec![]
        }
    }
}

impl IndexItem for Route {
    type Id = RouteId;

    fn model_id(&self) -> Self::Id {
        self.id()
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
    pub photos: Vec<Photo<RouteId>>,
    point_deltas: OnceCell<Vec<ElevationPointDelta>>,
    summary: OnceCell<SegmentSummary>,
}
impl RouteModel {
    pub fn new(
        route: Route,
        point_chunks: Vec<PointChunk<RouteId, ElevationPoint>>,
        photos: Vec<Photo<RouteId>>,
    ) -> RouteModel {
        RouteModel {
            route,
            point_chunks,
            photos,
            point_deltas: OnceCell::new(),
            summary: OnceCell::new(),
        }
    }

    pub fn iter_elevation_points(&self) -> impl Iterator<Item = &ElevationPoint> + '_ {
        PointChunk::iter_points(&self.point_chunks)
    }

    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.iter_elevation_points().map(|point| point.point)
    }

    pub fn point_deltas(&self) -> &[ElevationPointDelta] {
        self.point_deltas
            .get_or_init(|| generate_point_deltas(self.iter_elevation_points()))
            .as_slice()
    }

    pub fn segment_summary(&self) -> &SegmentSummary {
        self.summary
            .get_or_init(|| summarize_segment(self.point_deltas()))
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
        self.route.id()
    }

    fn as_index(&self) -> &Self::IndexItem {
        &self.route
    }

    fn into_parts(self) -> (Self::IndexItem, Vec<Self::OtherItem>) {
        (
            self.route,
            [
                self.point_chunks
                    .into_iter()
                    .map(RouteItem::from)
                    .collect_vec(),
                self.photos.into_iter().map(RouteItem::from).collect(),
            ]
            .concat(),
        )
    }

    fn from_parts(
        route: Self::IndexItem,
        other: Vec<Self::OtherItem>,
    ) -> Result<Self, anyhow::Error> {
        let photos = other
            .iter()
            .filter_map(RouteItem::as_photo)
            .cloned()
            .collect();

        Ok(RouteModel::new(
            route,
            other
                .clone()
                .into_iter()
                .filter_map(RouteItem::into_point_chunk)
                .collect(),
            photos,
        ))
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, From)]
#[serde(tag = "item")]
pub enum RouteItem {
    PointChunk(PointChunk<RouteId, ElevationPoint>),
    Photo(Photo<RouteId>),
}
impl RouteItem {
    pub fn as_point_chunk(&self) -> Option<&PointChunk<RouteId, ElevationPoint>> {
        match self {
            RouteItem::PointChunk(chunk) => Some(chunk),
            _ => None,
        }
    }
    pub fn as_photo(&self) -> Option<&Photo<RouteId>> {
        match self {
            RouteItem::Photo(photo) => Some(photo),
            _ => None,
        }
    }
    pub fn into_point_chunk(self) -> Option<PointChunk<RouteId, ElevationPoint>> {
        match self {
            RouteItem::PointChunk(chunk) => Some(chunk),
            _ => None,
        }
    }
    pub fn into_photo(self) -> Option<Photo<RouteId>> {
        match self {
            RouteItem::Photo(photo) => Some(photo),
            _ => None,
        }
    }
}
impl crate::models::OtherItem for RouteItem {
    type Id = RouteId;

    fn item_name(&self) -> String {
        match self {
            RouteItem::PointChunk(_) => "POINT_CHUNK".to_string(),
            RouteItem::Photo(_) => "PHOTO".to_string(),
        }
    }

    fn model_id(&self) -> RouteId {
        match self {
            RouteItem::PointChunk(chunk) => chunk.model_id,
            RouteItem::Photo(photo) => photo.model_id,
        }
    }

    fn item_id(&self) -> String {
        match self {
            RouteItem::PointChunk(chunk) => chunk.idx.to_string(),
            RouteItem::Photo(photo) => photo.id.as_ulid().to_string(),
        }
    }
}
