use serde::{Deserialize, Serialize};

use super::{
    point::{elevation_point, progress::DistanceElevationProgress},
    point_of_interest::PointOfInterest,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuesheet {
    pub cues: Vec<Cue>,
}

#[derive(Debug, Serialize, Deserialize, derive_more::Display)]
pub enum CueStop {
    #[display("Start")]
    Start,
    #[display("End")]
    End,
    #[display("{} ({})", "_0.name", "_0.point_of_interest_type")]
    POI(PointOfInterest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cue {
    pub origin: CueStop,
    pub destination: CueStop,
    pub summary: DistanceElevationProgress<elevation_point::ElevationPoint>,
}
