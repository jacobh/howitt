use super::point_of_interest::PointOfInterest;

pub struct Cuesheet {
    pub cues: Vec<Cue>,
}

pub enum CueStop {
    Start,
    End,
    POI(PointOfInterest),
}

pub struct Cue {
    pub origin: CueStop,
    pub destination: CueStop,
    pub distance_m: f64,
    pub vertical_ascent_m: f64,
    pub vertical_descent_m: f64,
}
