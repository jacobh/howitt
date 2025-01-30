use itertools::Itertools;

use crate::models::{
    point_of_interest::PointOfInterest,
    segment::{Segment, SegmentId},
};

use super::nearby::nearby_points_of_interest;

pub fn detect_segments(route: &gpx::Route, pois: &[PointOfInterest]) -> Vec<Segment> {
    let points = route.linestring().into_points();

    nearby_points_of_interest(&points, pois, 500.0)
        .into_iter()
        .sorted_by_key(|cp| cp.point_idx)
        .combinations(2)
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .map(|(cp1, cp2)| Segment {
            id: SegmentId::new(),
            start: cp1.point_of_interest.into_owned(),
            end: cp2.point_of_interest.into_owned(),
            route: gpx::Route {
                points: route.points[cp1.point_idx..=cp2.point_idx].to_vec(),
                ..route.clone()
            },
        })
        .collect()
}
