pub mod point {
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

pub mod compressed_bytes {
    use serde::{de::DeserializeOwned, Deserialize, Serialize};

    pub fn serialize<T: Serialize, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut encoder = snap::raw::Encoder::new();

        let bytes = bincode::serialize(value).map_err(serde::ser::Error::custom)?;
        let compressed_bytes = encoder
            .compress_vec(&bytes)
            .map_err(serde::ser::Error::custom)?;
        compressed_bytes.serialize(serializer)
    }

    pub fn deserialize<'de, T: DeserializeOwned, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut decoder = snap::raw::Decoder::new();

        let compressed_bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        let bytes = decoder
            .decompress_vec(&compressed_bytes)
            .map_err(serde::de::Error::custom)?;
        bincode::deserialize(&bytes).map_err(serde::de::Error::custom)
    }
}
