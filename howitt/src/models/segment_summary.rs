use serde::{Deserialize, Serialize};

use super::{point::Point, terminus::Termini};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SegmentSummary<P: Point> {
    pub distance_m: f64,
    pub elevation: Option<ElevationSummary>,
    pub termini: Termini<P>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElevationSummary {
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}
