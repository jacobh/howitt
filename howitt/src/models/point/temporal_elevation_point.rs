use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use super::{elevation_point::ElevationPoint, point::Point, point_delta::ElevationDelta};

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
