use std::{borrow::Cow, collections::HashMap};

use crate::models::{
    point::{ElevationPoint, Point, PointDelta},
    point_of_interest::PointOfInterest,
    route::Route,
    terminus::{ Termini, Terminus, TerminusEnd},
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

type NearbyRoute<'a> = (&'a Route, Terminus<ElevationPoint>, PointDelta);

pub fn routes_near_termini<'a, P: Point>(
    terimini: &Termini<P>,
    routes: &'a [Route],
) -> (Vec<NearbyRoute<'a>>, Vec<NearbyRoute<'a>>) {
    let (start, end) = terimini.points();

    let routes_near_start = routes_near_point(start, routes);
    let routes_near_end = routes_near_point(end, routes);

    let mut grouped_nearby_routes: HashMap<TerminusEnd, Vec<NearbyRoute>> = HashMap::from_iter(
        vec![]
            .into_iter()
            .chain(
                routes_near_start
                    .map(|(route, terminus, delta)| (route, terminus, delta, TerminusEnd::Start)),
            )
            .chain(
                routes_near_end
                    .map(|(route, terminus, delta)| (route, terminus, delta, TerminusEnd::End)),
            )
            .sorted_by_key(|(_, _, delta, _)| delta.distance as usize)
            .unique_by(|(route, terminus, _, _)| (route.id(), terminus.end))
            .group_by(|(_, _, _, end)| *end)
            .into_iter()
            .map(|(end, group)| {
                (
                    end,
                    group
                        .map(|(route, terminus, delta, _)| (route, terminus, delta))
                        .collect_vec(),
                )
            }),
    );

    (
        grouped_nearby_routes
            .remove(&TerminusEnd::Start)
            .unwrap_or_default(),
        grouped_nearby_routes
            .remove(&TerminusEnd::End)
            .unwrap_or_default(),
    )
}

pub fn routes_near_point<'a, P: Point>(
    point: &P,
    routes: &'a [Route],
) -> impl Iterator<Item = NearbyRoute<'a>> {
    routes
        .into_iter()
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
