use std::collections::HashSet;

use derive_more::From;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    models::{external_ref::ExternalRef, point::ElevationPoint},
    services::nearby::{nearby_routes, NearbyRoute},
};

use super::{
    external_ref::ExternallySourced,
    photo::Photo,
    point::PointChunk,
    route_description::RouteDescription,
    tag::Tag,
    terminus::{Termini, TerminusEnd},
    user::UserId,
    IndexItem, ModelName, ModelUuid,
};

pub type RouteId = ModelUuid<{ ModelName::Route }>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: RouteId,
    pub name: String,
    pub user_id: UserId,
    pub distance: f64,
    pub sample_points: Option<Vec<ElevationPoint>>,
    pub description: Option<RouteDescription>,
    pub external_ref: Option<ExternalRef>,
    #[serde(default)]
    pub tags: HashSet<Tag>,
}

impl Route {
    pub fn id(&self) -> RouteId {
        self.id
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

    pub fn sample_points(&self) -> impl Iterator<Item = &ElevationPoint> {
        self.sample_points.iter().flatten()
    }

    pub fn nearby_routes<'a, 'b>(&'a self, routes: &'b [Route]) -> Vec<NearbyRoute<'a, 'b>> {
        nearby_routes(self, routes)
    }

    pub fn routes_near_terminus<'a, 'b>(
        &'a self,
        routes: &'b [Route],
        end: TerminusEnd,
    ) -> Vec<NearbyRoute<'a, 'b>> {
        if let Some(termini) = self.termini() {
            self.nearby_routes(routes)
                .into_iter()
                .filter(|(point, _, _, _)| termini.closest_terminus(point).end == end)
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
pub struct RouteFilter {
    pub is_starred: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct RouteModel {
    pub route: Route,
    pub point_chunks: Vec<PointChunk<RouteId, ElevationPoint>>,
    pub photos: Vec<Photo<RouteId>>,
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
        }
    }

    pub fn iter_elevation_points(&self) -> impl Iterator<Item = &ElevationPoint> + '_ {
        PointChunk::iter_points(&self.point_chunks)
    }

    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.iter_elevation_points().map(|point| point.point)
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
    type Filter = RouteFilter;

    fn id(&self) -> RouteId {
        self.route.id()
    }

    fn as_index(&self) -> &Self::IndexItem {
        &self.route
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
            RouteItem::Photo(photo) => photo.id.as_uuid().to_string(),
        }
    }
}
