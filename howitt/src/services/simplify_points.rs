use geo::prelude::*;
use geo::LineString;

use crate::models::point::Point;

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
    max_points: usize,
    state: Option<SimplifyState>,
) -> LineString {
    if linestring.coords_count() <= max_points {
        return linestring;
    }

    let SimplifyState {
        epsilon: (lower, upper),
        i,
        oversimplified,
    } = state.unwrap_or_default();

    let epsilon = (lower + upper.unwrap_or(0.0005 * f64::powi(2.0, (i + 1) as i32))) / 2.0;

    let simplified = Simplify::simplify(&linestring, &epsilon);
    let count = simplified.coords_count();

    if count == max_points {
        // bang on target, return simplified
        simplified
    } else if i >= 50 {
        oversimplified.unwrap_or(simplified)
    } else if count > max_points {
        // too many points
        simplify_linestring(
            linestring,
            max_points,
            Some(SimplifyState {
                epsilon: (epsilon, upper),
                oversimplified,
                i: i + 1,
            }),
        )
    } else if i > 0 {
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
            max_points,
            Some(SimplifyState {
                epsilon: (lower, Some(epsilon)),
                i: i + 1,
                oversimplified,
            }),
        )
    } else {
        // initial epsilon was good enough to get us below the limit
        simplified
    }
}

pub fn simplify_points<P: Point>(points: &[P], target_points: usize) -> Vec<P> {
    let linestring = LineString::from_iter(points.iter().map(|p| *p.as_geo_point()));

    let simplified = simplify_linestring(linestring, target_points, None);

    simplified
        .points()
        .filter_map(|point| points.iter().find(|p| p.as_geo_point() == &point))
        .cloned()
        .collect()
}
