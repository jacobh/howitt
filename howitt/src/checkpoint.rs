use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::{Item, Model};

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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(with = "crate::serde_ext::point_tuple")]
    pub point: geo::Point<f64>,
    pub checkpoint_type: CheckpointType,
}

impl Model for Checkpoint {
    type Item = Checkpoint;

    fn model_name() -> &'static str {
        "CHECKPOINT"
    }

    fn id(&self) -> String {
        self.id.to_string()
    }

    fn into_items(self) -> impl Iterator<Item = Self::Item> {
        vec![self].into_iter()
    }

    fn from_items(items: Vec<Self::Item>) -> Result<Self, anyhow::Error> {
        items.into_iter().nth(0).ok_or(anyhow!("no items"))
    }
}

impl Item for Checkpoint {
    fn item_name(&self) -> Option<String> {
        None
    }

    fn model_id(&self) -> String {
        self.id.to_string()
    }

    fn item_id(&self) -> Option<String> {
        None
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
        match value.name.clone() {
            Some(name) => Ok(Checkpoint {
                id: uuid::Uuid::new_v4(),
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
