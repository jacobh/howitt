use std::fmt;

use chrono::{DateTime, Duration, Utc};
use derive_more::Constructor;
use geo::{geodesic_distance::GeodesicDistance, geodesic_length::GeodesicLength};
use gpx::TrackSegment;

use crate::gpx_ext::WaypointExt;
use crate::EtrexFile;

#[derive(Constructor)]
pub struct EtrexTrip {
    pub days: Vec<TripDay>,
}
impl EtrexTrip {
    fn add_segment(&mut self, segment: TrackSegment) {
        if segment
            .points
            .first()
            .and_then(|point| point.time())
            .unwrap()
            .signed_duration_since(self.end_time().unwrap())
            < Duration::hours(6)
        {
            self.days.last_mut().unwrap().segments.push(segment)
        } else {
            self.days.push(TripDay::new(vec![segment]))
        }
    }
    fn distance(&self) -> f64 {
        self.days.iter().map(|day| day.distance()).sum()
    }
    fn elapsed_time(&self) -> chrono::Duration {
        match (self.start_time(), self.end_time()) {
            (Some(start_time), Some(end_time)) => end_time.signed_duration_since(start_time),
            _ => chrono::Duration::seconds(0),
        }
    }
    fn start_waypoint(&self) -> Option<&gpx::Waypoint> {
        self.days.first().and_then(|day| day.start_waypoint())
    }
    fn end_waypoint(&self) -> Option<&gpx::Waypoint> {
        self.days.last().and_then(|day| day.end_waypoint())
    }
    fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_waypoint().and_then(|waypoint| waypoint.time())
    }
    fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end_waypoint().and_then(|waypoint| waypoint.time())
    }
}
impl fmt::Debug for EtrexTrip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EtrexTrip: {{ days: {}, distance: {}km, elapsed_time: {}h }}",
            self.days.len(),
            ((self.distance() / 100.0).round()) / 10.0,
            self.elapsed_time().num_hours(),
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
    fn start_waypoint(&self) -> Option<&gpx::Waypoint> {
        self.segments
            .first()
            .and_then(|segment| segment.points.first())
    }
    fn end_waypoint(&self) -> Option<&gpx::Waypoint> {
        self.segments
            .last()
            .and_then(|segment| segment.points.last())
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

pub fn detect_trips(mut files: Vec<EtrexFile>) -> Vec<EtrexTrip> {
    files.sort_by(|a, b| a.partial_cmp(b).unwrap());

    files
        .into_iter()
        .flat_map(|file| file.gpx.tracks)
        .fold(vec![], |mut accum, track| {
            let track_first_waypoint = track
                .segments
                .first()
                .and_then(|segment| segment.points.first());

            match (track_first_waypoint, accum.last_mut()) {
                (Some(track_first_waypoint), Some(prev_trip)) => {
                    let trip_end_waypoint = prev_trip.end_waypoint().unwrap();
                    let distance_meters = track_first_waypoint
                        .point()
                        .geodesic_distance(&trip_end_waypoint.point());

                    if distance_meters < 100.0 {
                        for segment in track.segments {
                            prev_trip.add_segment(segment)
                        }
                    } else {
                        accum.push(EtrexTrip::new(vec![TripDay::new(track.segments)]));
                    }
                }
                _ => {
                    accum.push(EtrexTrip::new(vec![TripDay::new(track.segments)]));
                }
            }
            accum
        })
}
