use serde::{Deserialize, Serialize};

use super::{
    point::{ElevationPoint, Point, PointDelta},
    slope_end::SlopeEnd,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum TerminusEnd {
    Start,
    End,
}

impl TerminusEnd {
    fn tuple_value<T>(&self, (start, end): (T, T)) -> T {
        match self {
            TerminusEnd::Start => start,
            TerminusEnd::End => end,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerminusElevation {
    pub slope_end: SlopeEnd,
    pub elevation_gain_from_start: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Terminus<P: Point> {
    pub termini: Termini<P>,
    pub end: TerminusEnd,
}

impl<P: Point> Terminus<P> {
    pub fn point(&self) -> &P {
        self.end.tuple_value(self.termini.points())
    }
    pub fn bearing(&self) -> f64 {
        let delta = &self.termini.delta();

        self.end
            .tuple_value(((delta.bearing + 180.0) % 360.0, delta.bearing))
    }
    pub fn distance_from_start(&self) -> f64 {
        let delta = &self.termini.delta();

        self.end.tuple_value((0.0, delta.distance))
    }
}

impl Terminus<ElevationPoint> {
    pub fn elevation(&self) -> TerminusElevation {
        let delta = &self.termini.delta();

        TerminusElevation {
            slope_end: self.end.tuple_value(SlopeEnd::from_delta(delta)),
            elevation_gain_from_start: self.end.tuple_value((0.0, delta.data.elevation_gain)),
        }
    }
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

    pub fn map_points<Q, F: Fn(P) -> Q>(self, f: F) -> Termini<Q> {
        let Termini {
            first_point,
            last_point,
        } = self;

        Termini {
            first_point: f(first_point),
            last_point: f(last_point),
        }
    }

    pub fn points(&self) -> (&P, &P) {
        (&self.first_point, &self.last_point)
    }

    pub fn into_points_array(self) -> [P; 2] {
        [self.first_point, self.last_point]
    }

    pub fn into_points(self) -> (P, P) {
        (self.first_point, self.last_point)
    }

    pub fn delta(&self) -> PointDelta<<P as Point>::DeltaData> {
        PointDelta::from_points(&self.first_point, &self.last_point)
    }

    pub fn to_termini(&self) -> (Terminus<P>, Terminus<P>) {
        (
            Terminus {
                termini: self.clone(),
                end: TerminusEnd::Start,
            },
            Terminus {
                termini: self.clone(),
                end: TerminusEnd::End,
            },
        )
    }

    pub fn to_termini_vec(&self) -> Vec<Terminus<P>> {
        let (a, b) = self.to_termini();
        vec![a, b]
    }

    pub fn closest_terminus<P1: Point>(&self, point: P1) -> Terminus<P> {
        self.to_termini_vec()
            .into_iter()
            .min_by_key(|t| PointDelta::from_points(point.as_geo_point(), t.point().as_geo_point()))
            .unwrap_or_else(|| self.to_termini().0)
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
