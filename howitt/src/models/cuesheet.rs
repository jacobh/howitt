use serde::{Deserialize, Serialize};

use super::point_of_interest::PointOfInterest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuesheet {
    pub cues: Vec<Cue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CueStop {
    Start,
    End,
    POI(PointOfInterest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cue {
    pub origin: CueStop,
    pub destination: CueStop,
    pub distance_m: f64,
    pub vertical_ascent_m: Option<f64>,
    pub vertical_descent_m: Option<f64>,
}
