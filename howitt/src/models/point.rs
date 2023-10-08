use chrono::{DateTime, TimeZone, Utc};
use geo::{CoordsIter, GeodesicBearing, LineString, Simplify};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::ModelId;

pub trait Point: std::fmt::Debug + Clone {
    fn as_geo_point(&self) -> &geo::Point;
    fn elevation_meters(&self) -> Option<&f64>;
    fn to_elevation_point(&self) -> Option<ElevationPoint>;

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
    fn as_geo_point(&self) -> &geo::Point {
        self
    }

    fn elevation_meters(&self) -> Option<&f64> {
        None
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        None
    }
}

impl<'a> Point for &'a geo::Point {
    fn as_geo_point(&self) -> &geo::Point {
        self
    }

    fn elevation_meters(&self) -> Option<&f64> {
        None
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        None
    }
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
    fn as_geo_point(&self) -> &geo::Point {
        &self.point
    }

    fn elevation_meters(&self) -> Option<&f64> {
        Some(&self.elevation)
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        Some(self.clone())
    }
}

impl<'a> Point for &'a ElevationPoint {
    fn as_geo_point(&self) -> &geo::Point {
        &self.point
    }

    fn elevation_meters(&self) -> Option<&f64> {
        Some(&self.elevation)
    }

    fn to_elevation_point(&self) -> Option<ElevationPoint> {
        Some((*self).clone())
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
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointChunk<ID, P> {
    pub model_id: ID,
    pub idx: usize,
    pub points: Vec<P>,
}
impl<ID, P> PointChunk<ID, P>
where
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PointDelta {
    pub distance: f64,
    pub bearing: f64,
    pub elevation_gain: Option<f64>,
}

impl PointDelta {
    pub fn zero() -> PointDelta {
        PointDelta {
            distance: 0.0,
            bearing: 0.0,
            elevation_gain: None,
        }
    }

    pub fn from_points<P1: Point, P2: Point>(p1: &P1, p2: &P2) -> PointDelta {
        let (bearing, distance) = p1
            .as_geo_point()
            .geodesic_bearing_distance(*p2.as_geo_point());

        let bearing = bearing % 360.0;

        let elevation_gain = match (p1.elevation_meters(), p2.elevation_meters()) {
            (Some(e1), Some(e2)) => Some(e2 - e1),
            _ => None,
        };

        PointDelta {
            distance,
            bearing,
            elevation_gain,
        }
    }
    pub fn from_points_tuple<P: Point>((p1, p2): (&P, &P)) -> PointDelta {
        PointDelta::from_points(p1, p2)
    }
}

pub fn generate_point_deltas<'a, P: Point + 'a>(
    points: impl IntoIterator<Item = &'a P>,
) -> Vec<PointDelta> {
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

// const LOWER_EPSILON: f64 = 0.0;
// const UPPER_EPSILON: f64 = 0.001;

// #[derive(Debug, Clone, Copy)]
// pub enum UpperEpsilon {
//     Attempts(usize),
//     Value(f64),
// }
// impl UpperEpsilon {
//     fn value(&self) -> f64 {
//         match self {
//             UpperEpsilon::Attempts(i) => ,
//             UpperEpsilon::Value(x) => *x,
//         }
//     }
//     fn increment_attempts(self) -> UpperEpsilon {
//         match self {
//             UpperEpsilon::Attempts(i) => UpperEpsilon::Attempts(i + 1),
//             UpperEpsilon::Value(y) => UpperEpsilon::Value(y),
//         }
//     }
// }

pub struct SimplifyState {
    epsilon: (f64, Option<f64>),
    i: usize,
    oversimplified: Option<LineString>,
}

impl Default for SimplifyState {
    fn default() -> Self {
        Self {
            epsilon: (0.0, None),
            oversimplified: Default::default(),
            i: 0,
        }
    }
}

pub fn simplify_linestring(
    linestring: LineString,
    max_points: usize,
    state: Option<SimplifyState>,
) -> LineString {
    if linestring.coords_count() <= max_points {
        return linestring;
    }

    let SimplifyState {
        epsilon: (lower, upper),
        i,
        oversimplified,
    } = state.unwrap_or_default();

    let epsilon = (lower + upper.unwrap_or(0.0005 * f64::powi(2.0, (i + 1) as i32))) / 2.0;

    let simplified = Simplify::simplify(&linestring, &epsilon);
    let count = simplified.coords_count();

    if count == max_points {
        // bang on target, return simplified
        simplified
    } else if i >= 50 {
        oversimplified.unwrap_or(simplified)
    } else if count > max_points {
        // too many points
        simplify_linestring(
            linestring,
            max_points,
            Some(SimplifyState {
                epsilon: (epsilon, upper),
                oversimplified,
                i: i + 1,
            }),
        )
    } else {
        if i > 0 {
            let oversimplified = oversimplified.unwrap_or(LineString::new(vec![]));
            let oversimplified = Some(
                if simplified.coords_count() > oversimplified.coords_count() {
                    simplified
                } else {
                    oversimplified
                },
            );

            // not enough points and we're not using the default epsilon. search our way back to the limit
            simplify_linestring(
                linestring,
                max_points,
                Some(SimplifyState {
                    epsilon: (lower, Some(epsilon)),
                    i: i + 1,
                    oversimplified,
                }),
            )
        } else {
            // initial epsilon was good enough to get us below the limit
            simplified
        }
    }
}

pub fn simplify_points<P: Point>(points: &[P], target_points: usize) -> Vec<P> {
    let linestring = LineString::from_iter(points.iter().map(|p| *p.as_geo_point()));

    let simplified = simplify_linestring(linestring, target_points, None);

    simplified
        .points()
        .map(|point| points.iter().find(|p| p.as_geo_point() == &point))
        .flatten()
        .cloned()
        .collect()
}
