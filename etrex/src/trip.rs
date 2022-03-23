use std::fmt;

use derive_more::Constructor;
use geo::geodesic_length::GeodesicLength;
use gpx::TrackSegment;

use crate::EtrexFile;

#[derive(Constructor)]
pub struct EtrexTrip {
    pub days: Vec<TripDay>,
}
impl EtrexTrip {
    fn distance(&self) -> f64 {
        self.days.iter().map(|day| day.distance()).sum()
    }
}
impl fmt::Debug for EtrexTrip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EtrexTrip: {{ days: {}, distance: {}km }}",
            self.days.len(),
            ((self.distance() / 100.0).round()) / 10.0
        )
    }
}

#[derive(Constructor)]
pub struct TripDay {
    pub segments: Vec<TrackSegment>,
}
impl TripDay {
    fn distance(&self) -> f64 {
        self.segments
            .iter()
            .map(|segment| segment.linestring().geodesic_length())
            .sum()
    }
}
impl fmt::Debug for TripDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TripDay: {{ distance: {}km }}",
            ((self.distance() / 100.0).round()) / 10.0
        )
    }
}

pub fn detect_trips(files: Vec<EtrexFile>) -> Vec<EtrexTrip> {
    files
        .into_iter()
        .flat_map(|file| file.gpx.tracks)
        .fold(vec![], |mut accum, track| {
            accum.push(EtrexTrip::new(vec![TripDay::new(track.segments)]));
            accum
        })
}
