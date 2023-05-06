use itertools::Itertools;

use crate::models::{
    point_of_interest::PointOfInterest,
    segment::{Segment, SegmentId},
};

use super::nearby::nearby_points_of_interest;

pub fn detect_segments(route: &gpx::Route, pois: &[PointOfInterest]) -> Vec<Segment> {
    nearby_points_of_interest(route, pois)
        .into_iter()
        .sorted_by_key(|cp| cp.point_idx)
        .filter(|cp| cp.distance < 500.0)
        .combinations(2)
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .map(|(cp1, cp2)| Segment {
            id: SegmentId::new(),
            start: cp1.point_of_interest.clone(),
            end: cp2.point_of_interest.clone(),
            route: gpx::Route {
                points: route.points[cp1.point_idx..=cp2.point_idx].to_vec(),
                ..route.clone()
            },
        })
        .collect()
}
