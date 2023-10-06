use geo::GeodesicDistance;
use serde::{Deserialize, Serialize};

use super::point::{ElevationPoint, Point};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SlopeEnd {
    Uphill,
    Downhill,
    Flat,
}

impl SlopeEnd {
    pub fn from_points(p1: ElevationPoint, p2: ElevationPoint) -> (SlopeEnd, SlopeEnd) {
        let delta = p2.elevation - p1.elevation;
        // let delta_i = delta.to_isize().unwrap()
        let distance = GeodesicDistance::geodesic_distance(p1.as_geo_point(), p2.as_geo_point());
        let gradient_percent = 100.0 / distance * delta;

        if delta.abs() <= 25.0 || gradient_percent.abs() < 0.5 {
            (SlopeEnd::Flat, SlopeEnd::Flat)
        } else if delta > 0.0 {
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
