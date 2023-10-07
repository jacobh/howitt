use serde::{Deserialize, Serialize};

use super::{
    cardinal_direction::CardinalDirection,
    point::{Point, PointDelta},
    slope_end::SlopeEnd,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerminusElevation {
    pub slope_end: SlopeEnd,
    pub elevation_gain_from_start: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Terminus<P: Point> {
    pub direction: CardinalDirection,
    pub point: P,
    pub distance_from_start: f64,
    pub elevation: Option<TerminusElevation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
        let delta = PointDelta::from_points(&self.first_point, &self.last_point);

        let slope_ends = SlopeEnd::from_delta(&delta);

        let (start_elevation, end_elevation) = match (delta.elevation_gain, slope_ends) {
            (Some(elevation_gain_from_start), Some((end1, end2))) => (
                Some(TerminusElevation {
                    slope_end: end1,
                    elevation_gain_from_start: 0.0,
                }),
                Some(TerminusElevation {
                    slope_end: end2,
                    elevation_gain_from_start,
                }),
            ),
            _ => (None, None),
        };

        (
            Terminus {
                direction: CardinalDirection::from_bearing(delta.bearing).inverse(),
                distance_from_start: 0.0,
                point: self.first_point.clone(),
                elevation: start_elevation,
            },
            Terminus {
                direction: CardinalDirection::from_bearing(delta.bearing),
                distance_from_start: f64::round(delta.distance * 100.0) / 100.0,
                point: self.last_point.clone(),
                elevation: end_elevation,
            },
        )
    }

    pub fn to_termini_vec(&self) -> Vec<Terminus<P>> {
        let (a, b) = self.to_termini();
        vec![a, b]
    }
}

#[cfg(test)]
mod tests {
    use crate::models::point::ElevationPoint;

    use super::*;

    #[test]
    fn to_termini_works() {
        let point1 = ElevationPoint {
            point: geo::Point::new(146.60587, -37.2154),
            elevation: 1100.0,
        };
        let point2 = ElevationPoint {
            point: geo::Point::new(146.68021, -37.20515),
            elevation: 1400.0,
        };

        let termini = Termini::new(point1.clone(), point2.clone());

        let result = termini.to_termini();

        insta::assert_debug_snapshot!(result)
    }
}
