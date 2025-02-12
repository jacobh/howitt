use crate::services::euclidean::iter_geo_to_euclidean;
use derive_more::derive::Display;
use geo::prelude::*;
use geo::LineString;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::models::point::Point;

#[derive(Debug, Display, Clone)]
pub enum DetailLevel {
    #[display("XLOW")]
    ExtremelyLow,
    #[display("LOW")]
    Low,
    #[display("MED")]
    Medium,
    #[display("HIGH")]
    High,
    #[display("CUSTOM#{}", _0)]
    Custom(f64),
}

impl DetailLevel {
    /// Area threshold (m²) below which a point is removed in the Visvalingam-Whyatt algorithm
    pub fn epsilon(&self) -> f64 {
        match self {
            DetailLevel::ExtremelyLow => 50_000.0,
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
