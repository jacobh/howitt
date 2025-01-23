use geo::GeodesicBearing;
use serde::{Deserialize, Serialize};

use crate::services::num::Round2;

use super::{
    super::segment_summary::{ElevationSummary, SummaryData},
    point::Point,
};

pub trait DeltaData: Default + Ord + Clone + Round2 {
    type SummaryData: SummaryData;

    fn to_summary(&self) -> Self::SummaryData;
}

impl DeltaData for () {
    type SummaryData = ();
    fn to_summary(&self) -> Self::SummaryData {}
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Round2)]
pub struct ElevationDelta {
    pub elevation_gain: f64,
}

impl DeltaData for ElevationDelta {
    type SummaryData = ElevationSummary;

    fn to_summary(&self) -> Self::SummaryData {
        ElevationSummary {
            elevation_ascent_m: if self.elevation_gain > 0.0 {
                self.elevation_gain
            } else {
                0.0
            },
            elevation_descent_m: if self.elevation_gain < 0.0 {
                self.elevation_gain.abs()
            } else {
                0.0
            },
        }
    }
}

impl Eq for ElevationDelta {}

impl Ord for ElevationDelta {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn o_f(x: f64) -> ordered_float::OrderedFloat<f64> {
            ordered_float::OrderedFloat(x)
        }

        o_f(self.elevation_gain).cmp(&o_f(other.elevation_gain))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Round2)]
pub struct PointDelta<T>
where
    T: DeltaData,
{
    pub distance: f64,
    pub bearing: f64,
    pub data: T,
}

pub type ElevationPointDelta = PointDelta<ElevationDelta>;

impl<T> PointDelta<T>
where
    T: DeltaData,
{
    pub fn zero() -> PointDelta<T> {
        PointDelta {
            distance: 0.0,
            bearing: 0.0,
            data: T::default(),
        }
    }

    pub fn from_points<P: Point<DeltaData = T>>(p1: &P, p2: &P) -> PointDelta<T> {
        let (bearing, distance) = p1
            .as_geo_point()
            .geodesic_bearing_distance(*p2.as_geo_point());

        let bearing = (bearing + 360.0) % 360.0;

        PointDelta {
            distance,
            bearing,
            data: P::delta(p1, p2),
        }
    }
    pub fn from_points_tuple<P: Point<DeltaData = T>>((p1, p2): (&P, &P)) -> PointDelta<T> {
        PointDelta::from_points(p1, p2)
    }
}

impl<T> Eq for PointDelta<T> where T: DeltaData + Eq {}

impl<T> Ord for PointDelta<T>
where
    T: DeltaData + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn o_f(x: f64) -> ordered_float::OrderedFloat<f64> {
            ordered_float::OrderedFloat(x)
        }

        o_f(self.distance)
            .cmp(&o_f(other.distance))
            .then(o_f(self.bearing).cmp(&o_f(other.bearing)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::point::{ElevationDelta, ElevationPoint, PointDelta},
        services::num::Round2,
    };

    #[test]
    fn test_delta_basic() {
        let p1 = ElevationPoint {
            point: geo::Point::new(146.55653, -37.1722),
            elevation: 831.1,
        };

        let p2 = ElevationPoint {
            point: geo::Point::new(146.52389, -37.21561),
            elevation: 1495.7,
        };

        let delta = PointDelta::from_points(&p1, &p2);

        assert_eq!(
            delta.round2(),
            PointDelta {
                distance: 5622.13,
                bearing: 211.02,
                data: ElevationDelta {
                    elevation_gain: 664.6
                }
            }
        )
    }
}
