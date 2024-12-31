pub mod point_tuple {
    use serde::{Deserialize, Serialize};

    pub fn serialize<S>(point: &geo::Point<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        point.x_y().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<geo::Point<f64>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let x_y: (f64, f64) = Deserialize::deserialize(deserializer)?;
        Ok(geo::Point::new(x_y.0, x_y.1))
    }
}

pub mod json {
    pub fn into_string_value(value: serde_json::Value) -> Option<String> {
        match value {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn unwrap_string_value(value: serde_json::Value) -> String {
        into_string_value(value).unwrap()
    }
}
