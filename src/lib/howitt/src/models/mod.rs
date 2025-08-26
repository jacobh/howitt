use std::{hash::Hash, marker::ConstParamTy};

use serde::{Deserialize, Serialize};
use uuid::Timestamp;

pub mod cardinal_direction;
pub mod config;
pub mod cuesheet;
pub mod external_ref;
pub mod filters;
pub mod maybe_pair;
pub mod media;
pub mod note;
pub mod osm;
pub mod point;
pub mod point_of_interest;
pub mod point_of_interest_visit;
pub mod ride;
pub mod route;
pub mod route_description;
pub mod segment_summary;
pub mod slope_end;
pub mod tag;
pub mod terminus;
pub mod trip;
pub mod user;

pub trait Model: Send + Sync + Sized + Clone + 'static {
    type Id: ModelId;
    type Filter: Send + Sync + Sized + Clone + 'static;

    fn id(&self) -> Self::Id;
}

pub trait ModelId:
    Send + Sync + std::fmt::Debug + std::fmt::Display + PartialEq + Copy + Clone + Hash + Eq + 'static
{
    fn model_name() -> &'static str;
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum ModelName {
    Media,
    PointOfInterest,
    PointOfInterestVisit,
    Ride,
    Route,
    User,
    Trip,
    Note,
    OsmFeature,
}
impl ModelName {
    const fn to_str(self) -> &'static str {
        match self {
            ModelName::Media => "MEDIA",
            ModelName::PointOfInterest => "POI",
            ModelName::PointOfInterestVisit => "POI_VISIT",
            ModelName::Ride => "RIDE",
            ModelName::Route => "ROUTE",
            ModelName::User => "USER",
            ModelName::Trip => "TRIP",
            ModelName::Note => "NOTE",
            ModelName::OsmFeature => "OSM_FEATURE",
        }
    }
}

#[derive(derive_more::From, derive_more::Into, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ModelUuid<const NAME: ModelName>(uuid::Uuid);

impl<const NAME: ModelName> ModelUuid<NAME> {
    pub fn new() -> ModelUuid<NAME> {
        ModelUuid::<NAME>(uuid::Uuid::now_v7())
    }
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
    pub fn from_datetime(datetime: chrono::DateTime<chrono::Utc>) -> ModelUuid<NAME> {
        ModelUuid::<NAME>(uuid::Uuid::new_v7(
            Timestamp::from_unix(
                uuid::timestamp::context::ContextV7::new(),
                datetime.timestamp() as u64,
                0,
            )
            .into(),
        ))
    }
    pub fn get_or_from_datetime(
        id: Option<ModelUuid<NAME>>,
        datetime: &chrono::DateTime<chrono::Utc>,
    ) -> ModelUuid<NAME> {
        match id {
            Some(id) => id,
            None => ModelUuid::<NAME>::from_datetime(*datetime),
        }
    }
}

impl<const NAME: ModelName> ModelId for ModelUuid<NAME> {
    fn model_name() -> &'static str {
        NAME.to_str()
    }
}

impl<const NAME: ModelName> std::fmt::Debug for ModelUuid<NAME> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", NAME.to_str(), self.0)
    }
}

impl<const NAME: ModelName> std::fmt::Display for ModelUuid<NAME> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", NAME.to_str(), self.0)
    }
}

impl<const NAME: ModelName> Default for ModelUuid<NAME> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const NAME: ModelName> Serialize for ModelUuid<NAME> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de, const NAME: ModelName> Deserialize<'de> for ModelUuid<NAME> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let model_id: String = Deserialize::deserialize(deserializer)?;
        let parts = model_id.split('#').collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(serde::de::Error::custom("expected 2 parts"));
        }

        let name = parts[0];
        let id = parts[1];

        if name != NAME.to_str() {
            return Err(serde::de::Error::custom(
                "model name component of ID did not match",
            ));
        }

        std::str::FromStr::from_str(id)
            .map(ModelUuid::<NAME>)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_model_uuid_from_datetime_preserves_timestamp() {
        // Create a fixed datetime for testing
        let dt = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let uuid = ModelUuid::<{ ModelName::Route }>::from_datetime(dt);

        // Convert the UUID to string and check the format
        let uuid_str = uuid.to_string();
        assert!(uuid_str.starts_with("ROUTE#"));

        // The first 48 bits of the UUID should represent the Unix timestamp
        let timestamp = dt.timestamp() as u64;
        let (uuid_timestamp_secs, _subsec_nanos) =
            uuid.as_uuid().get_timestamp().unwrap().to_unix();
        assert_eq!(timestamp, uuid_timestamp_secs);
    }
    #[test]
    fn test_model_uuid_from_datetime_ordering() {
        // Create two datetimes with known ordering
        let dt1 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let dt2 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 1).unwrap();

        let uuid1 = ModelUuid::<{ ModelName::Route }>::from_datetime(dt1);
        let uuid2 = ModelUuid::<{ ModelName::Route }>::from_datetime(dt2);

        // UUIDs should maintain chronological ordering
        assert!(uuid1.as_uuid() < uuid2.as_uuid());

        // Both should have the correct prefix
        assert!(uuid1.to_string().starts_with("ROUTE#"));
        assert!(uuid2.to_string().starts_with("ROUTE#"));
    }
}
