use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::ModelId;

pub trait Point: std::fmt::Debug + Clone {
    fn as_geo_point(&self) -> &geo::Point;
    fn elevation_meters(&self) -> Option<&f64>;
    fn to_elevation_point(&self) -> Option<ElevationPoint>;
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
