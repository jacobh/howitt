use thiserror::Error;

use crate::gtfs::GtfsStop;

#[derive(Debug)]
pub struct Checkpoint {
    pub name: String,
    pub point: geo::Point<f64>,
}

impl From<crate::gtfs::GtfsStop> for Checkpoint {
    fn from(value: GtfsStop) -> Self {
        let GtfsStop { name, point, ..} = value;
        Checkpoint { name, point }
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
            Some(name) => Ok(Checkpoint { name, point: value.point() }),
            None => Err(CheckpointError::MissingName)
        }
    }
}