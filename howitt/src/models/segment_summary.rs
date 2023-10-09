use howitt_derive::Round2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Round2)]
pub struct SegmentSummary {
    pub distance_m: f64,
    pub elevation: Option<ElevationSummary>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Round2)]
pub struct ElevationSummary {
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}
