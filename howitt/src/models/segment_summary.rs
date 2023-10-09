use serde::{Deserialize, Serialize};

use crate::services::num::round2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SegmentSummary {
    pub distance_m: f64,
    pub elevation: Option<ElevationSummary>,
}

impl SegmentSummary {
    pub fn round2(self) -> SegmentSummary {
        let SegmentSummary {
            distance_m,
            elevation,
        } = self;

        SegmentSummary {
            distance_m: round2(distance_m),
            elevation: elevation.map(ElevationSummary::round2),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElevationSummary {
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}

impl ElevationSummary {
    pub fn round2(self) -> ElevationSummary {
        let ElevationSummary {
            elevation_ascent_m,
            elevation_descent_m,
        } = self;

        ElevationSummary {
            elevation_ascent_m: round2(elevation_ascent_m),
            elevation_descent_m: round2(elevation_descent_m),
        }
    }
}
