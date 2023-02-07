use serde::{Deserialize, Serialize};

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
