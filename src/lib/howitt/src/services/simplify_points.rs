use crate::services::euclidean::iter_geo_to_euclidean;
use derive_more::derive::Display;
use geo::prelude::*;
use geo::LineString;
use itertools::Itertools;
use rustc_hash::FxHashMap;

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

pub enum DetailLevel {
    Low,
    Medium,
    High,
    Custom(f64),
}

impl DetailLevel {
    /// Area threshold (m²) below which a point is removed in the Visvalingam-Whyatt algorithm
    pub fn epsilon(&self) -> f64 {
        match self {
            DetailLevel::Low => 10_000.0,
            DetailLevel::Medium => 600.0,
            DetailLevel::High => 20.0,
            DetailLevel::Custom(e) => *e,
        }
    }
}

pub fn simplify_points_v2<P: Point>(points: Vec<P>, detail_level: DetailLevel) -> Vec<P> {
    if points.is_empty() {
        return Vec::new();
    }

    let epsilon = detail_level.epsilon();

    // Convert points to geo points
    let geo_points = points.iter().map(|p| *p.as_geo_point());
    let euclidean_points = iter_geo_to_euclidean(geo_points).collect_vec();

    // Create a HashMap mapping ordered (x,y) tuples to original points
    let mut point_map: FxHashMap<_, _> = points
        .into_iter()
        .zip(euclidean_points.iter())
        .map(|(p, e)| (e.ordered_x_y(), p))
        .collect();

    // Create LineString from euclidean points
    let euclidean_linestring = LineString::from(euclidean_points);

    // Simplify the euclidean linestring
    let simplified_linestring = SimplifyVw::simplify_vw(&euclidean_linestring, &epsilon);

    // Use HashMap lookup with ordered x,y coordinates
    simplified_linestring
        .points()
        .filter_map(|p| point_map.remove(&p.ordered_x_y()))
        .collect()
}
