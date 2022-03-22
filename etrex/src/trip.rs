use derive_more::Constructor;
use gpx::TrackSegment;

#[derive(Constructor)]
pub struct EtrexTrip {
    pub days: Vec<TripDay>,
}

#[derive(Constructor)]
pub struct TripDay {
    pub segments: Vec<TrackSegment>,
}
