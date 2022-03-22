use geo::geodesic_length::GeodesicLength;
use std::fmt;
use thiserror::Error;

struct WaypointRef<'a> {
    track: &'a gpx::Track,
    segment: &'a gpx::TrackSegment,
    waypoint: &'a gpx::Waypoint,
}
impl<'a> WaypointRef<'a> {
    fn new(
        track: &'a gpx::Track,
        segment: &'a gpx::TrackSegment,
        waypoint: &'a gpx::Waypoint,
    ) -> WaypointRef<'a> {
        WaypointRef {
            track,
            segment,
            waypoint,
        }
    }
}

#[derive(Error, Debug)]
#[error("Data parse failed")]
pub struct EtrexParseError {
    #[from]
    gpx_error: gpx::errors::GpxError,
}

pub struct EtrexFile {
    gpx: gpx::Gpx,
}
impl EtrexFile {
    pub fn parse(data: &[u8]) -> Result<EtrexFile, EtrexParseError> {
        let gpx = gpx::read(data)?;
        Ok(EtrexFile { gpx })
    }
    fn waypoints<'a>(&'a self) -> impl Iterator<Item = WaypointRef<'a>> {
        self.gpx.tracks.iter().flat_map(|track| {
            track.segments.iter().flat_map(move |segment| {
                segment
                    .points
                    .iter()
                    .map(move |waypoint| WaypointRef::new(track, segment, waypoint))
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
