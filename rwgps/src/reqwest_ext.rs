use serde::de::DeserializeOwned;

use crate::RwgpsError;

#[derive(thiserror::Error, Debug)]
#[error("{url}")]
pub struct SerdeDebugError {
    url: String,
    #[source]
    error: serde_path_to_error::Error<serde_json::Error>,
}

pub trait ResponseExt {
    async fn json_debug<T: DeserializeOwned>(self) -> Result<T, RwgpsError>;
}

impl ResponseExt for reqwest::Response {
    async fn json_debug<T: DeserializeOwned>(self) -> Result<T, RwgpsError> {
        let url = self.url().to_string();

        let bytes = self.bytes().await?;

        let jd = &mut serde_json::Deserializer::from_slice(&bytes);

        match serde_path_to_error::deserialize(jd) {
            Ok(val) => Ok(val),
            Err(error) => Err(RwgpsError::from(SerdeDebugError { url, error })),
        }
    }
}
