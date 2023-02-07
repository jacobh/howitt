pub mod point {
    use serde::{Serialize, Deserialize};

    pub fn serialize<S>(point: &geo::Point<f64>, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        point.x_y().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<geo::Point<f64>, D::Error> where D: serde::Deserializer<'de> {
        let x_y: (f64, f64) = Deserialize::deserialize(deserializer)?;
        Ok(geo::Point::new(x_y.0, x_y.1))
    }
}