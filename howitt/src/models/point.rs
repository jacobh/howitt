use chrono::{DateTime, TimeZone, Utc};
use geo::GeodesicBearing;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::services::num::Round2;

use super::{
    segment_summary::{ElevationSummary, SummaryData},
    ModelId,
};

pub trait Point: std::fmt::Debug + Clone {
    type DeltaData: DeltaData;

    fn as_geo_point(&self) -> &geo::Point;
    fn elevation_meters(&self) -> Option<&f64>;
    fn to_elevation_point(&self) -> Option<ElevationPoint>;
    fn delta(&self, other: &Self) -> Self::DeltaData;

    fn x_y(&self) -> (f64, f64) {
        geo::Point::x_y(*self.as_geo_point())
    }

    fn ordered_x_y(
        &self,
    ) -> (
        ordered_float::OrderedFloat<f64>,
        ordered_float::OrderedFloat<f64>,
    ) {
        let (x, y) = self.x_y();

        (
            ordered_float::OrderedFloat(x),
            ordered_float::OrderedFloat(y),
        )
    }

    fn x_y_z(&self) -> (f64, f64, Option<f64>) {
        let (x, y) = self.x_y();

        (x, y, self.elevation_meters().copied())
    }

    fn to_x_y_vec(&self) -> Vec<f64> {
        let (x, y) = self.x_y();
        vec![x, y]
    }

    fn into_x_y_vec(self) -> Vec<f64> {
        let (x, y) = self.x_y();
        vec![x, y]
    }
}

impl Point for geo::Point {
    type DeltaData = ();

    fn as_geo_point(&self) -> &geo::Point {
        self
    }

    fn elevation_meters(&self) -> Option<&f64> {
        None
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        None
    }

    fn delta(&self, _: &Self) -> Self::DeltaData {}
}

impl<'a> Point for &'a geo::Point {
    type DeltaData = ();

    fn as_geo_point(&self) -> &geo::Point {
        self
    }

    fn elevation_meters(&self) -> Option<&f64> {
        None
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        None
    }

    fn delta(&self, _: &Self) -> Self::DeltaData {}
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElevationPoint {
    pub point: geo::Point,
    pub elevation: f64,
}

impl Serialize for ElevationPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.point.x(), self.point.y(), self.elevation).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ElevationPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y, elevation) = Deserialize::deserialize(deserializer)?;
        Ok(ElevationPoint {
            point: geo::Point::new(x, y),
            elevation,
        })
    }
}

impl Point for ElevationPoint {
    type DeltaData = ElevationDelta;

    fn as_geo_point(&self) -> &geo::Point {
        &self.point
    }

    fn elevation_meters(&self) -> Option<&f64> {
        Some(&self.elevation)
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        Some(self.clone())
    }

    fn delta(&self, other: &Self) -> Self::DeltaData {
        ElevationDelta {
            elevation_gain: other.elevation - self.elevation,
        }
    }
}

impl<'a> Point for &'a ElevationPoint {
    type DeltaData = ElevationDelta;

    fn as_geo_point(&self) -> &geo::Point {
        &self.point
    }

    fn elevation_meters(&self) -> Option<&f64> {
        Some(&self.elevation)
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        Some((*self).clone())
    }

    fn delta(&self, other: &Self) -> Self::DeltaData {
        ElevationDelta {
            elevation_gain: other.elevation - self.elevation,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TemporalElevationPoint {
    pub datetime: DateTime<Utc>,
    pub point: geo::Point,
    pub elevation: f64,
}

impl Serialize for TemporalElevationPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (
            self.datetime.timestamp(),
            self.point.x(),
            self.point.y(),
            self.elevation,
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TemporalElevationPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (timestamp, x, y, elevation) = Deserialize::deserialize(deserializer)?;
        Ok(TemporalElevationPoint {
            datetime: Utc
                .timestamp_opt(timestamp, 0)
                .single()
                .ok_or_else(|| serde::de::Error::custom("couldnt parse timestamp"))?,
            point: geo::Point::new(x, y),
            elevation,
        })
    }
}

impl Point for TemporalElevationPoint {
    type DeltaData = ElevationDelta;

    fn as_geo_point(&self) -> &geo::Point {
        &self.point
    }

    fn elevation_meters(&self) -> Option<&f64> {
        Some(&self.elevation)
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        Some(ElevationPoint {
            point: self.point,
            elevation: self.elevation,
        })
    }

    fn delta(&self, other: &Self) -> Self::DeltaData {
        ElevationDelta {
            elevation_gain: other.elevation - self.elevation,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointChunk<ID, P> {
    pub model_id: ID,
    pub idx: usize,
    pub points: Vec<P>,
}
impl<ID, P> PointChunk<ID, P>
where
    P: 'static,
    ID: ModelId,
{
    pub fn new_chunks(model_id: ID, points: impl IntoIterator<Item = P>) -> Vec<PointChunk<ID, P>> {
        points
            .into_iter()
            .chunks(2500)
            .into_iter()
            .enumerate()
            .map(|(idx, points)| PointChunk {
                model_id,
                idx,
                points: points.collect(),
            })
            .collect()
    }
    pub fn iter_points(chunks: &[PointChunk<ID, P>]) -> impl Iterator<Item = &P> + '_ {
        chunks.iter().flat_map(|chunk| chunk.points.iter())
    }
    pub fn into_iter_points(chunks: Vec<PointChunk<ID, P>>) -> impl Iterator<Item = P> + 'static {
        chunks
            .into_iter()
            .flat_map(|chunk| chunk.points.into_iter())
    }
}

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

pub type SimplePointDelta = PointDelta<ElevationDelta>;
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

pub fn generate_point_deltas<'a, P>(
    points: impl IntoIterator<Item = &'a P>,
) -> Vec<PointDelta<P::DeltaData>>
where
    P: Point + 'a,
{
    let mut is_points_empty = true;

    let deltas = points
        .into_iter()
        .inspect(|_| is_points_empty = false)
        .tuple_windows()
        .map(PointDelta::from_points_tuple)
        .collect_vec();

    // add an empty delta to the start to keep this aligned with the input
    if !is_points_empty {
        [vec![PointDelta::zero()], deltas].concat()
    } else {
        deltas
    }
}

pub fn closest_point<'a, P: Point>(
    point: &P,
    points: impl Iterator<Item = &'a P>,
) -> Option<(&'a P, PointDelta<P::DeltaData>)> {
    points
        .map(|p| {
            let delta = PointDelta::from_points(point, p);
            (p, delta)
        })
        .min_by_key(|(_, delta)| delta.clone())
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
