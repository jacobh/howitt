use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use gtfs::GtfsStop;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: uuid::Uuid,
    pub name: String,
    pub point: geo::Point<f64>,
    pub checkpoint_type: CheckpointType,
}

impl From<GtfsStop> for Checkpoint {
    fn from(value: GtfsStop) -> Self {
        let GtfsStop {
            stop_name,
            stop_lat,
            stop_lon,
            ..
        } = value;
        Checkpoint {
            id: uuid::Uuid::new_v4(),
            name: stop_name,
            point: geo::Point::new(stop_lon, stop_lat),
            checkpoint_type: CheckpointType::RailwayStation,
        }
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
