use derive_more::derive::Display;
use geo::prelude::*;
use geo::LineString;

use crate::models::point::Point;

#[derive(Debug, Display, Clone)]
pub enum SimplifyTarget {
    #[display("TOTAL_POINTS#{}", _0)]
    TotalPoints(usize),
    #[display("POINTS_PER_KM#{}", _0)]
    PointPerKm(usize),
}

pub struct SimplifyState {
    epsilon: (f64, Option<f64>),
    i: usize,
    oversimplified: Option<LineString>,
}

impl Default for SimplifyState {
    fn default() -> Self {
        Self {
            epsilon: (0.0, None),
            oversimplified: Default::default(),
            i: 0,
        }
    }
}

pub fn simplify_linestring(
    linestring: LineString,
    target: SimplifyTarget,
    state: Option<SimplifyState>,
) -> LineString {
    let max_points = match target {
        SimplifyTarget::TotalPoints(points) => points,
        SimplifyTarget::PointPerKm(points_per_km) => {
            let length_km = linestring.length::<Haversine>() / 1000.0;

            (length_km * (points_per_km as f64)) as usize
        }
    };

    if linestring.coords_count() <= max_points {
        return linestring;
    }

    let SimplifyState {
        epsilon: (lower, upper),
        i,
        oversimplified,
    } = state.unwrap_or_default();

    let epsilon = (lower + upper.unwrap_or(0.0001 * f64::powi(2.0, (i + 1) as i32))) / 2.0;

    let simplified = Simplify::simplify(&linestring, &epsilon);
    let count = simplified.coords_count();

    if count == max_points {
        // bang on target, return simplified
        simplified
    } else if i >= 20 {
        oversimplified.unwrap_or(simplified)
    } else if count > max_points {
        // too many points
        simplify_linestring(
            linestring,
            SimplifyTarget::TotalPoints(max_points),
            Some(SimplifyState {
                epsilon: (epsilon, upper),
                oversimplified,
                i: i + 1,
            }),
        )
    } else {
        let oversimplified = oversimplified.unwrap_or(LineString::new(vec![]));
        let oversimplified = Some(
            if simplified.coords_count() > oversimplified.coords_count() {
                simplified
            } else {
                oversimplified
            },
        );

        // not enough points and we're not using the default epsilon. search our way back to the limit
        simplify_linestring(
            linestring,
            SimplifyTarget::TotalPoints(max_points),
            Some(SimplifyState {
                epsilon: (lower, Some(epsilon)),
                i: i + 1,
                oversimplified,
            }),
        )
    }
}

pub fn simplify_points<P: Point>(points: &[P], target: SimplifyTarget) -> Vec<P> {
    let linestring = LineString::from_iter(points.iter().map(|p| *p.as_geo_point()));

    let simplified = simplify_linestring(linestring, target, None);

    simplified
        .points()
        .filter_map(|point| points.iter().find(|p| p.as_geo_point() == &point))
        .cloned()
        .collect()
}
