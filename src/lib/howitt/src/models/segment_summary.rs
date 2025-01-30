use crate::services::num::Round2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Round2)]
pub struct SegmentSummary<T: SummaryData> {
    pub distance_m: f64,
    pub data: T,
}

pub trait SummaryData: Default + Round2 {
    fn fold(self, other: Self) -> Self;
}

impl SummaryData for () {
    fn fold(self, _other: Self) -> Self {}
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Round2)]
pub struct ElevationSummary {
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}

impl SummaryData for ElevationSummary {
    fn fold(self, other: Self) -> Self {
        ElevationSummary {
            elevation_ascent_m: self.elevation_ascent_m + other.elevation_ascent_m,
            elevation_descent_m: self.elevation_descent_m + other.elevation_descent_m,
        }
    }
}

pub type SegmentElevationSummary = SegmentSummary<ElevationSummary>;
