use serde::{Deserialize, Serialize};

use super::point::{ElevationDelta, ElevationPointDelta, PointDelta};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SlopeEnd {
    Uphill,
    Downhill,
    Flat,
}

impl SlopeEnd {
    pub fn from_delta(
        PointDelta {
            distance,
            data: ElevationDelta { elevation_gain },
            ..
        }: &ElevationPointDelta,
    ) -> (SlopeEnd, SlopeEnd) {
        let gradient_percent = 100.0 / distance * elevation_gain;

        if elevation_gain.abs() <= 25.0 || gradient_percent.abs() < 0.5 {
            (SlopeEnd::Flat, SlopeEnd::Flat)
        } else if elevation_gain > &0.0 {
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
