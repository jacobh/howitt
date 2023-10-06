use geo::{GeodesicBearing, GeodesicDistance};
use num::ToPrimitive;
use serde::{Deserialize, Serialize};

use super::point::Point;

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
    pub fn from_points<P: Point>(p1: P, p2: P) -> Option<(SlopeEnd, SlopeEnd)> {
        match (p1.elevation_meters(), p2.elevation_meters()) {
            (Some(e1), (Some(e2))) => {
                let delta = e2 - e1;
                // let delta_i = delta.to_isize().unwrap()
                let distance =
                    GeodesicDistance::geodesic_distance(p1.as_geo_point(), p2.as_geo_point());
                let gradient_percent = 100.0 / distance * delta;

                if delta.abs() <= 25.0 || gradient_percent.abs() < 0.5 {
                    Some((SlopeEnd::Flat, SlopeEnd::Flat))
                } else if delta > 0.0 {
                    Some((SlopeEnd::Downhill, SlopeEnd::Uphill))
                } else {
                    Some((SlopeEnd::Uphill, SlopeEnd::Downhill))
                }
            }
            _ => None,
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
pub struct Terminus<P: Point> {
    pub direction: CardinalDirection,
    pub slope_end: Option<SlopeEnd>,
    pub point: P,
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

    pub fn points(&self) -> (&P, Option<&P>) {
        let first_to_last_distance = GeodesicDistance::geodesic_distance(
            self.first_point.as_geo_point(),
            self.last_point.as_geo_point(),
        );

        if first_to_last_distance < 100.0 {
            (&self.first_point, None)
        } else {
            (&self.first_point, Some(&self.last_point))
        }
    }

    pub fn termini(&self) -> Option<(Terminus<&P>, Terminus<&P>)> {
        match self.points() {
            (first_point, Some(last_point)) => {
                let first_to_last_bearing = GeodesicBearing::geodesic_bearing(
                    first_point.as_geo_point(),
                    *last_point.as_geo_point(),
                );

                let (start_slope, end_slope) = match SlopeEnd::from_points(first_point, last_point)
                {
                    Some((a, b)) => (Some(a), Some(b)),
                    None => (None, None),
                };

                Some((
                    Terminus {
                        direction: CardinalDirection::from_bearing(first_to_last_bearing).inverse(),
                        slope_end: start_slope,
                        point: first_point,
                    },
                    Terminus {
                        direction: CardinalDirection::from_bearing(first_to_last_bearing),
                        slope_end: end_slope,
                        point: last_point,
                    },
                ))
            }
            (_, None) => None,
        }
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
