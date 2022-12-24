use thiserror::Error;

use gtfs::GtfsStop;

#[derive(Debug)]
pub struct Checkpoint {
    pub name: String,
    pub point: geo::Point<f64>,
}

impl From<GtfsStop> for Checkpoint {
    fn from(value: GtfsStop) -> Self {
        let GtfsStop { stop_name, stop_lat, stop_lon, ..} = value;
        Checkpoint { name: stop_name, point: geo::Point::new(stop_lat, stop_lon) }
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