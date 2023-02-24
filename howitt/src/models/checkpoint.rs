use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{ext::ulid::generate_ulid, models::Model};

use super::IndexModel;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckpointType {
    RailwayStation,
    Hut,
    Locality,
    Generic,
}

impl FromStr for CheckpointType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RAILWAY_STATION" => Ok(CheckpointType::RailwayStation),
            "HUT" => Ok(CheckpointType::Hut),
            "LOCALITY" => Ok(CheckpointType::Locality),
            "GENERIC" => Ok(CheckpointType::Generic),
            _ => Err(()),
        }
    }
}

impl Display for CheckpointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CheckpointType::RailwayStation => "RAILWAY_STATION",
            CheckpointType::Hut => "HUT",
            CheckpointType::Locality => "LOCALITY",
            CheckpointType::Generic => "GENERIC",
        };

        f.write_str(s)
    }
}

crate::model_id!(CheckpointId, "CHECKPOINT");

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: ulid::Ulid,
    pub name: String,
    #[serde(with = "crate::ext::serde::point_tuple")]
    pub point: geo::Point<f64>,
    pub checkpoint_type: CheckpointType,
}

impl IndexModel for Checkpoint {
    type Id = CheckpointId;

    fn id(&self) -> CheckpointId {
        CheckpointId::from(self.id)
    }
}

#[derive(Debug, Error)]
#[error("Checkpoint conversion failed")]
pub enum CheckpointError {
    MissingName,
}

impl TryFrom<gpx::Waypoint> for Checkpoint {
    type Error = CheckpointError;
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
            Some(name) => Ok(Checkpoint {
                id,
                name,
                point: value.point(),
                checkpoint_type: value
                    ._type
                    .unwrap_or("".to_string())
                    .parse()
                    .unwrap_or(CheckpointType::Generic),
            }),
            None => Err(CheckpointError::MissingName),
        }
    }
}
