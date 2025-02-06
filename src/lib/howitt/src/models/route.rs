use std::collections::HashSet;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    models::{external_ref::ExternalRef, point::ElevationPoint},
    services::nearby::{nearby_routes, NearbyRoute},
};

use super::{
    external_ref::ExternallySourced,
    route_description::RouteDescription,
    tag::Tag,
    terminus::{Termini, TerminusEnd},
    user::UserId,
    IndexItem, IndexModel, ModelName, ModelUuid,
};

pub type RouteId = ModelUuid<{ ModelName::Route }>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: RouteId,
    pub name: String,
    pub slug: String,
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
pub enum RouteFilter {
    Starred,
    All,
    Slug(String),
}

#[derive(Debug, Clone)]
pub struct RouteModel {
    pub route: Route,
    pub points: Vec<ElevationPoint>,
}
impl RouteModel {
    pub fn new(route: Route) -> RouteModel {
        RouteModel {
            route,
            points: vec![],
        }
    }

    pub fn iter_elevation_points(&self) -> impl Iterator<Item = &ElevationPoint> + '_ {
        self.points.iter()
    }

    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.iter_elevation_points().map(|point| point.point)
    }
}

impl PartialEq for RouteModel {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route && self.points == other.points
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoutePoints {
    pub id: RouteId,
    pub points: Vec<ElevationPoint>,
}

impl RoutePoints {
    pub fn iter_elevation_points(&self) -> impl Iterator<Item = &ElevationPoint> + '_ {
        self.points.iter()
    }

    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.iter_elevation_points().map(|point| point.point)
    }
}

impl IndexModel for RoutePoints {
    type Id = RouteId;
    type Filter = ();

    fn id(&self) -> RouteId {
        RouteId::from(self.id)
    }
}
