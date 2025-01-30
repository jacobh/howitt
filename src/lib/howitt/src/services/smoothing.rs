use itertools::Itertools;

use crate::models::point::{
    delta::{AccumulatingDelta, DistanceDelta},
    ElevationPoint,
};

pub fn smooth_elevations(cum_distances: &[f64], elevations: &[f64]) -> Vec<f64> {
    if elevations.len() < 2 {
        return elevations.to_vec();
    }

    let spline = csaps::CubicSmoothingSpline::new(&cum_distances, &elevations)
        .with_smooth(0.000002)
        .make()
        .unwrap();

    let smoothed_elevations = spline.evaluate(&cum_distances).unwrap();

    smoothed_elevations.to_vec()
}

fn filter_duplicate_points(points: Vec<ElevationPoint>) -> Vec<ElevationPoint> {
    if points.is_empty() {
        return points;
    }

    let deltas = DistanceDelta::running_totals(&points);

    std::iter::zip(points, deltas)
        .with_position()
        .filter_map(
            |(position, (point, DistanceDelta(distance)))| match position {
                itertools::Position::First | itertools::Position::Only => Some(point),
                itertools::Position::Middle | itertools::Position::Last => {
                    if almost::zero(distance) {
                        None
                    } else {
                        Some(point)
                    }
                }
            },
        )
        .collect_vec()
}

pub fn smooth_elevation_points(points: Vec<ElevationPoint>) -> Vec<ElevationPoint> {
    let points = filter_duplicate_points(points);

    let deltas = DistanceDelta::running_totals(&points);
    let distances: Vec<f64> = deltas.into_iter().map(|DistanceDelta(d)| d).collect();

    let raw_elevations = points.iter().map(|point| point.elevation).collect_vec();

    std::iter::zip(points, smooth_elevations(&distances, &raw_elevations))
        .map(|(point, smoothed_elevation)| ElevationPoint {
            elevation: smoothed_elevation,
            ..point
        })
        .collect_vec()
}
