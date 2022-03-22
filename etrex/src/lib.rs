use derive_more::Constructor;
use geo::geodesic_length::GeodesicLength;
use gpx::{TrackSegment, Waypoint, Track};
use std::fmt;
use thiserror::Error;

mod trip;

#[derive(Error, Debug)]
#[error("Data parse failed")]
pub struct EtrexParseError {
    #[from]
    gpx_error: gpx::errors::GpxError,
}

#[derive(Constructor, Clone)]
pub struct EtrexFile {
    gpx: gpx::Gpx,
}
impl EtrexFile {
    pub fn parse(data: &[u8]) -> Result<EtrexFile, EtrexParseError> {
        let gpx = gpx::read(data)?;
        Ok(EtrexFile { gpx })
    }
    fn waypoints<'a>(&'a self) -> impl Iterator<Item = (&'a Track, &'a TrackSegment, &'a Waypoint)> {
        self.gpx.tracks.iter().flat_map(|track| {
            track.segments.iter().flat_map(move |segment| {
                segment
                    .points
                    .iter()
                    .map(move |waypoint| (track, segment, waypoint))
            })
        })
    }
    fn linestrings(&self) -> impl Iterator<Item = geo::LineString<f64>> + '_ {
        self.gpx
            .tracks
            .iter()
            .flat_map(|track| &track.segments)
            .map(|segment| segment.linestring())
    }
}

#[derive(Constructor, Debug)]
pub struct EtrexFileSet {
    pub files: Vec<EtrexFile>,
}
impl EtrexFileSet {
    pub fn trips(&self) -> impl Iterator<Item = trip::EtrexTrip> + '_ {
        self.files
            .iter()
            .cloned()
            .flat_map(|file| file.gpx.tracks)
            .map(|track| trip::EtrexTrip::new(vec![trip::TripDay::new(track.segments)]))
    }
}

impl fmt::Debug for EtrexFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let distance = self
            .linestrings()
            .map(|linestring| linestring.geodesic_length())
            .sum::<f64>()
            .round();

        write!(
            f,
            "EtrexFile: {{ tracks: {}, points: {}, distance: {}m }}",
            self.gpx.tracks.len(),
            self.waypoints().count(),
            distance
        )
    }
}
