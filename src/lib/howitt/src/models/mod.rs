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
pub mod point;
pub mod point_of_interest;
pub mod ride;
pub mod route;
pub mod route_description;
pub mod segment;
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

pub trait IndexModel {
    type Id: ModelId;
    type Filter: Send + Sync + Sized + Clone + 'static;

    fn id(&self) -> Self::Id;
}

impl<T, ID, F> Model for T
where
    T: IndexModel<Id = ID, Filter = F> + Send + Sync + Sized + Clone + 'static,
    ID: ModelId + 'static,
    F: Send + Sync + Sized + Clone + 'static,
{
    type Id = ID;

    type Filter = F;

    fn id(&self) -> Self::Id {
        self.id()
    }
}

pub trait ModelId:
    Send + Sync + std::fmt::Debug + std::fmt::Display + PartialEq + Copy + Clone + Hash + Eq + 'static
{
    fn model_name() -> &'static str;
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum ModelName {
    Media,
    Checkpoint,
    Ride,
    Route,
    Segment,
    User,
    Trip,
}
impl ModelName {
    const fn to_str(self) -> &'static str {
        match self {
            ModelName::Media => "MEDIA",
            ModelName::Checkpoint => "CHECKPOINT",
            ModelName::Ride => "RIDE",
            ModelName::Route => "ROUTE",
            ModelName::Segment => "SEGMENT",
            ModelName::User => "USER",
            ModelName::Trip => "TRIP",
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

// #[cfg(test)]
// mod tests {
//     use chrono::{DateTime, Utc};
//     use test_case::test_case;

//     use crate::models::route::RouteId;

//     fn datetime1() -> DateTime<Utc> {
//         DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
//             .unwrap()
//             .into()
//     }

//     fn datetime2() -> DateTime<Utc> {
//         DateTime::parse_from_rfc3339("2023-10-01T00:00:00Z")
//             .unwrap()
//             .into()
//     }

//     const ULID_PREFIX1: &str = "ROUTE#01GNNA1J00";
//     const ULID_PREFIX2: &str = "ROUTE#01HBM8HS00";

//     #[test_case(Some(RouteId::from_datetime(datetime1())), datetime2(), ULID_PREFIX1)]
//     #[test_case(Some(RouteId::from_datetime(datetime2())), datetime1(), ULID_PREFIX2)]
//     #[test_case(None, datetime2(), ULID_PREFIX2)]
//     fn test_get_or_from_datetime(
//         existing_id: Option<RouteId>,
//         datetime: DateTime<Utc>,
//         expected_prefix: &str,
//     ) {
//         let id = RouteId::get_or_from_datetime(existing_id, &datetime).to_string();
//         let (prefix, _) = id.split_at(expected_prefix.len());

//         assert_eq!(expected_prefix, prefix);
//     }
// }
