use itertools::Itertools;

use crate::models::point::{generate_point_deltas, ElevationPoint};

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
    let deltas = generate_point_deltas(points.iter());

    std::iter::zip(points, deltas)
        .with_position()
        .filter_map(|(position, (point, delta))| match position {
            itertools::Position::First | itertools::Position::Only => Some(point),
            itertools::Position::Middle | itertools::Position::Last => {
                if almost::zero(delta.distance) {
                    None
                } else {
                    Some(point)
                }
            }
        })
        .collect_vec()
}

pub fn smooth_elevation_points(points: Vec<ElevationPoint>) -> Vec<ElevationPoint> {
    let points = filter_duplicate_points(points);

    let deltas = generate_point_deltas(points.as_slice());

    let distances = deltas
        .iter()
        .map(|delta| delta.distance)
        .scan(0.0, |total_distance, distance| {
            *total_distance += distance;

            Some(*total_distance)
        })
        .collect_vec();

    let raw_elevations = points.iter().map(|point| point.elevation).collect_vec();

    std::iter::zip(
        points,
        smooth_elevations(&distances, &raw_elevations).into_iter(),
    )
    .map(|(point, smoothed_elevation)| ElevationPoint {
        elevation: smoothed_elevation,
        ..point
    })
    .collect_vec()
}
