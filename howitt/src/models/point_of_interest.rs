use std::{fmt::Display, str::FromStr};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ext::ulid::generate_ulid;

use super::IndexModel;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PointOfInterestType {
    RailwayStation,
    Hut,
    Locality,
    Generic,
}

impl FromStr for PointOfInterestType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RAILWAY_STATION" => Ok(PointOfInterestType::RailwayStation),
            "HUT" => Ok(PointOfInterestType::Hut),
            "LOCALITY" => Ok(PointOfInterestType::Locality),
            "GENERIC" => Ok(PointOfInterestType::Generic),
            _ => Err(()),
        }
    }
}

impl Display for PointOfInterestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PointOfInterestType::RailwayStation => "RAILWAY_STATION",
            PointOfInterestType::Hut => "HUT",
            PointOfInterestType::Locality => "LOCALITY",
            PointOfInterestType::Generic => "GENERIC",
        };

        f.write_str(s)
    }
}

crate::model_id!(PointOfInterestId, "CHECKPOINT");

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInterest {
    pub id: ulid::Ulid,
    pub name: String,
    #[serde(with = "crate::ext::serde::point_tuple")]
    pub point: geo::Point<f64>,
    #[serde(rename = "checkpoint_type")]
    pub point_of_interest_type: PointOfInterestType,
}

impl IndexModel for PointOfInterest {
    type Id = PointOfInterestId;

    fn id(&self) -> PointOfInterestId {
        PointOfInterestId::from(self.id)
    }
}

#[derive(Debug, Error)]
#[error("POI conversion failed")]
pub enum PointOfInterestError {
    MissingName,
}

impl TryFrom<gpx::Waypoint> for PointOfInterest {
    type Error = PointOfInterestError;
    fn try_from(value: gpx::Waypoint) -> Result<Self, Self::Error> {
        let waypoint_created_at = value
            .time
            .map(|time| time.format())
            .transpose()
            .unwrap()
            .as_deref()
            .map(DateTime::parse_from_rfc3339)
            .transpose()
            .unwrap();

        let id = generate_ulid(waypoint_created_at, &value).unwrap();

        match value.name.clone() {
            Some(name) => Ok(PointOfInterest {
                id,
                name,
                point: value.point(),
                point_of_interest_type: value
                    .type_
                    .unwrap_or("".to_string())
                    .parse()
                    .unwrap_or(PointOfInterestType::Generic),
            }),
            None => Err(PointOfInterestError::MissingName),
        }
    }
}
