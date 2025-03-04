use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use super::{point::Point, WithDatetime, WithElevation};

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
}

impl WithElevation for TemporalElevationPoint {
    fn elevation(&self) -> f64 {
        self.elevation
    }
}

impl WithDatetime for TemporalElevationPoint {
    fn datetime(&self) -> &DateTime<Utc> {
        &self.datetime
    }
}
