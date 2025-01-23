use serde::{Deserialize, Serialize};

use super::point::delta::{DistanceDelta, ElevationDelta};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SlopeEnd {
    Uphill,
    Downhill,
    Flat,
}

impl SlopeEnd {
    pub fn from_deltas(
        distance: &DistanceDelta,
        elevation_gain: &ElevationDelta,
    ) -> (SlopeEnd, SlopeEnd) {
        let gradient_percent = 100.0 / distance.0 * elevation_gain.0;

        if elevation_gain.0.abs() <= 25.0 || gradient_percent.abs() < 0.5 {
            (SlopeEnd::Flat, SlopeEnd::Flat)
        } else if elevation_gain.0 > 0.0 {
            (SlopeEnd::Downhill, SlopeEnd::Uphill)
        } else {
            (SlopeEnd::Uphill, SlopeEnd::Downhill)
        }
    }

    pub fn inverse(self) -> SlopeEnd {
        match self {
            SlopeEnd::Uphill => SlopeEnd::Downhill,
            SlopeEnd::Downhill => SlopeEnd::Uphill,
            SlopeEnd::Flat => SlopeEnd::Flat,
        }
    }
}
