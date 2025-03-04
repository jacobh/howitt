use std::borrow::Cow;

use crate::models::{
    point::{
        closest_point,
        delta::{BearingDelta, Delta, DistanceDelta, ElevationDelta},
        ElevationPoint, Point,
    },
    point_of_interest::PointOfInterest,
    route::Route,
};
use geo::{algorithm::line_measures::metric_spaces::Haversine, Distance};
use itertools::Itertools;

use super::simplify_points::{simplify_points_v2, DetailLevel};

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

impl<P> NearbyPointOfInterest<'_, '_, P>
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
                        Haversine::distance(*point.as_geo_point(), poi.point),
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

pub type NearbyRoute<'a, 'b> = (
    ElevationPoint,
    &'b Route,
    &'b ElevationPoint,
    (DistanceDelta, BearingDelta, ElevationDelta),
);

const MAX_DISTANCE: f64 = 25_000.0;

pub fn nearby_routes<'a, 'b>(route: &'a Route, routes: &'b [Route]) -> Vec<NearbyRoute<'a, 'b>> {
    let sample_points = route
        .sample_points
        .clone()
        .map(|points| simplify_points_v2(points, DetailLevel::ExtremelyLow))
        .unwrap_or_default();

    routes
        .iter()
        .filter(|route2| route.id() != route2.id())
        .flat_map(|route2| {
            sample_points
                .iter()
                .flat_map(move |sample_point| {
                    closest_point(sample_point, route2.sample_points()).map(
                        |(route2_point, distance)| {
                            let delta = (
                                distance,
                                BearingDelta::delta(sample_point, route2_point),
                                ElevationDelta::delta(sample_point, route2_point),
                            );
                            (sample_point.clone(), route2, route2_point, delta)
                        },
                    )
                })
                .min_by_key(|(_, _, _, (DistanceDelta(distance), _, _))| {
                    ordered_float::OrderedFloat(*distance)
                })
        })
        .filter(|(_, _, _, (DistanceDelta(distance), _, _))| *distance < MAX_DISTANCE)
        .collect_vec()
}

pub fn routes_near_point<'a, 'b>(
    point: &'a ElevationPoint,
    routes: &'b [Route],
) -> impl Iterator<Item = NearbyRoute<'a, 'b>> {
    routes
        .iter()
        .flat_map(move |route| {
            closest_point(point, route.sample_points()).map(|(route_point, distance)| {
                let delta = (
                    distance,
                    BearingDelta::delta(point, route_point),
                    ElevationDelta::delta(point, route_point),
                );
                (point.clone(), route, route_point, delta)
            })
        })
        .filter(|(_, _, _, (DistanceDelta(distance), _, _))| *distance < MAX_DISTANCE)
}
