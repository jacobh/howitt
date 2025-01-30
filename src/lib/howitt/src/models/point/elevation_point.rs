use serde::{Deserialize, Serialize};

use super::{point::Point, WithElevation};

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
}

impl Point for &ElevationPoint {
    fn as_geo_point(&self) -> &geo::Point {
        &self.point
    }
}

impl WithElevation for ElevationPoint {
    fn elevation(&self) -> f64 {
        self.elevation
    }
}

impl WithElevation for &ElevationPoint {
    fn elevation(&self) -> f64 {
        self.elevation
    }
}
