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