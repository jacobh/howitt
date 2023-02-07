use chrono::{DateTime, Utc};

pub trait WaypointExt {
    fn time(&self) -> Option<DateTime<Utc>>;
}

impl WaypointExt for gpx::Waypoint {
    fn time(&self) -> Option<DateTime<Utc>> {
        let time_string = self.time?.format().ok()?;
        return Some(DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339(&time_string).ok()?,
        ));
    }
}
