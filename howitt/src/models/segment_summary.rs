use geo::{GeodesicBearing, GeodesicDistance};
use num::ToPrimitive;
use serde::{Deserialize, Serialize};

use super::point::{ElevationPoint, Point};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}
impl CardinalDirection {
    pub fn from_bearing(bearing: impl ToPrimitive) -> CardinalDirection {
        match bearing.to_isize().unwrap() {
            ..45 => CardinalDirection::North,
            45..135 => CardinalDirection::East,
            135..225 => CardinalDirection::East,
            225..315 => CardinalDirection::East,
            315.. => CardinalDirection::North,
            _ => unreachable!(),
        }
    }
    pub fn inverse(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminusElevation {
    pub slope_end: SlopeEnd,
    pub elevation_gain_from_start: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terminus<P: Point> {
    pub direction: CardinalDirection,
    pub point: P,
    pub distance_from_start: f64,
    pub elevation: Option<TerminusElevation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Termini<P> {
    first_point: P,
    last_point: P,
}
impl<P: Point> Termini<P> {
    pub fn new(first_point: P, last_point: P) -> Termini<P> {
        Termini {
            first_point,
            last_point,
        }
    }

    pub fn points(&self) -> (&P, &P) {
        (&self.first_point, &self.last_point)
    }

    pub fn to_termini(&self) -> (Terminus<P>, Terminus<P>) {
        let (first_point, last_point) = self.points();

        let (first_to_last_bearing, first_to_last_distance) =
            GeodesicBearing::geodesic_bearing_distance(
                first_point.as_geo_point(),
                *last_point.as_geo_point(),
            );

        let (start_elevation, end_elevation) = match (
            first_point.into_elevation_point(),
            last_point.into_elevation_point(),
        ) {
            (Some(e1), Some(e2)) => {
                let elevation_gain_from_start = e2.elevation - e1.elevation;
                let (a, b) = SlopeEnd::from_points(e1, e2);
                (
                    Some(TerminusElevation {
                        slope_end: a,
                        elevation_gain_from_start: 0.0,
                    }),
                    Some(TerminusElevation {
                        slope_end: b,
                        elevation_gain_from_start,
                    }),
                )
            }
            _ => (None, None),
        };

        (
            Terminus {
                direction: CardinalDirection::from_bearing(first_to_last_bearing).inverse(),
                distance_from_start: 0.0,
                point: first_point.clone(),
                elevation: start_elevation,
            },
            Terminus {
                direction: CardinalDirection::from_bearing(first_to_last_bearing),
                distance_from_start: first_to_last_distance,
                point: last_point.clone(),
                elevation: end_elevation,
            },
        )
    }

    pub fn to_termini_vec(&self) -> Vec<Terminus<P>> {
        let (a, b) = self.to_termini();
        vec![a, b]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentSummary<P: Point> {
    pub distance_m: f64,
    pub elevation: Option<ElevationSummary>,
    pub termini: Termini<P>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ElevationSummary {
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}
