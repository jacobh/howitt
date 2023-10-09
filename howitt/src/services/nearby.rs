use std::{borrow::Cow, iter};

use crate::models::{
    point::{closest_point, ElevationPoint, Point, PointDelta},
    point_of_interest::PointOfInterest,
    route::Route,
};
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

pub type NearbyRoute<'a, 'b, P> = (&'a P, &'b Route, &'b ElevationPoint, PointDelta);

pub fn nearby_routes<'a, 'b>(
    route: &'a Route,
    routes: &'b [Route],
) -> Vec<NearbyRoute<'a, 'b, ElevationPoint>> {
    // let nearby_routes = route.sample_points.iter().flatten().map(|point| routes_near_point(point, routes));

    let (routes_near_start, routes_near_end) = match route.termini() {
        Some(termini) => {
            let (start, end) = termini.into_points();

            (
                Some(routes_near_point(start, routes)),
                Some(routes_near_point(end, routes)),
            )
        }
        None => (None, None),
    };

    let nearby_routes = iter::empty()
        .chain(routes_near_start.into_iter().flatten())
        .chain(routes_near_end.into_iter().flatten())
        .filter(|(_, route2, _, _)| route.id() != route2.id());

    let grouped = nearby_routes.group_by(|(_, route, _, _)| route.id());

    grouped
        .into_iter()
        .flat_map(|(_, group)| {
            group.sorted_by_key(|(_, _, _, delta)| delta.clone()).fold(
                Vec::new(),
                |mut nearby_routes, (point, route, route_point, delta)| {
                    let is_separated = nearby_routes.iter().all(|(_, _, nearby_point, _)| {
                        let delta = PointDelta::from_points(route_point, nearby_point);

                        delta.distance > 5000.0
                    });

                    if is_separated {
                        nearby_routes.push((point, route, route_point, delta))
                    }

                    nearby_routes
                },
            )
        })
        .collect_vec()
}

pub fn routes_near_point<'a, 'b, P: Point>(
    point: &'a P,
    routes: &'b [Route],
) -> impl Iterator<Item = NearbyRoute<'a, 'b, P>> {
    routes
        .iter()
        .flat_map(move |route| {
            closest_point(point, route.sample_points.iter().flatten())
                .map(|(route_point, delta)| (point, route, route_point, delta))
        })
        .filter(|(_, _, _, delta)| delta.distance < 25_000.0)
}
