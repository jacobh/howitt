use std::{borrow::Cow, collections::HashMap, iter};

use crate::models::{
    point::{ElevationPoint, Point, PointDelta},
    point_of_interest::PointOfInterest,
    route::Route,
    terminus::{Terminus, TerminusEnd},
};
use either::Either;
use geo::algorithm::haversine_distance::HaversineDistance;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct NearbyPointOfInterest<'point, 'poi, P>
where
    P: Point + std::fmt::Debug + ToOwned,
    <P as ToOwned>::Owned: std::fmt::Debug,
{
    pub point_idx: usize,
    pub closest_point: Cow<'point, P>,
    pub distance: f64,
    pub point_of_interest: Cow<'poi, PointOfInterest>,
}

impl<'point, 'poi, P> NearbyPointOfInterest<'point, 'poi, P>
where
    P: Point + std::fmt::Debug + ToOwned,
    <P as ToOwned>::Owned: std::fmt::Debug,
{
    pub fn into_owned(self) -> NearbyPointOfInterest<'static, 'static, P> {
        NearbyPointOfInterest {
            closest_point: Cow::Owned(self.closest_point.into_owned()),
            point_of_interest: Cow::Owned(self.point_of_interest.into_owned()),
            ..self
        }
    }
}

pub fn nearby_points_of_interest<'a, 'b, P>(
    route: &'a [P],
    pois: &'b [PointOfInterest],
    max_distance_m: f64,
) -> Vec<NearbyPointOfInterest<'a, 'b, P>>
where
    P: Point + std::fmt::Debug + ToOwned,
    <P as ToOwned>::Owned: std::fmt::Debug,
{
    pois.iter()
        .filter_map(|poi| {
            let closest_point = route
                .iter()
                .enumerate()
                .map(|(i, point)| {
                    (
                        i,
                        point,
                        point.as_geo_point().haversine_distance(&poi.point),
                    )
                })
                .filter(|(_, _, distance)| max_distance_m >= *distance)
                .min_by_key(|(_, _, distance)| ordered_float::OrderedFloat(*distance));

            closest_point.map(
                |(point_idx, closest_point, distance)| NearbyPointOfInterest {
                    point_idx,
                    closest_point: Cow::Borrowed(closest_point),
                    distance,
                    point_of_interest: Cow::Borrowed(poi),
                },
            )
        })
        .collect()
}

pub type NearbyRoute<'a> = (&'a Route, Terminus<ElevationPoint>, PointDelta);

pub fn nearby_routes<'a>(
    route: &Route,
    routes: &'a [Route],
) -> (Vec<NearbyRoute<'a>>, Vec<NearbyRoute<'a>>) {
    let (routes_near_start, routes_near_end) = match &route.termini {
        Some(termini) => {
            let (start, end) = termini.points();

            (
                Some(routes_near_point(start, routes)),
                Some(routes_near_point(end, routes)),
            )
        }
        None => (None, None),
    };

    let nearby_routes = iter::empty()
        .chain(
            routes_near_start
                .into_iter()
                .flatten()
                .map(|nearby_route| (TerminusEnd::Start, nearby_route)),
        )
        .chain(
            routes_near_end
                .into_iter()
                .flatten()
                .map(|nearby_route| (TerminusEnd::End, nearby_route)),
        )
        .filter(|(_, (route2, _, _))| route.id() != route2.id())
        .sorted_by_key(|(_, (_, _, delta))| delta.distance as usize)
        .unique_by(|(_, (route, terminus, _))| (route.id(), terminus.end));

    nearby_routes.partition_map(|(end, nearby_route)| match end {
        TerminusEnd::Start => Either::Left(nearby_route),
        TerminusEnd::End => Either::Right(nearby_route),
    })
}

pub fn routes_near_point<'a, P: Point>(
    point: &P,
    routes: &'a [Route],
) -> impl Iterator<Item = NearbyRoute<'a>> {
    routes
        .iter()
        .flat_map(|route| {
            route
                .termini
                .as_ref()
                .map(|t| t.to_termini_vec())
                .unwrap_or_default()
                .into_iter()
                .map(|terminus| {
                    let delta = PointDelta::from_points(point, &terminus.point);
                    (route, terminus, delta)
                })
                .collect_vec()
        })
        .sorted_by_key(|(_, _, delta)| delta.distance as usize)
        .take_while(|(_, _, delta)| delta.distance < 25_000.0)
        .unique_by(|(route, _, _)| route.id())
}
