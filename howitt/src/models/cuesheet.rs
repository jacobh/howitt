use serde::{Deserialize, Serialize};

use super::{point_of_interest::PointOfInterest, segment_summary::SegmentSummary};

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
    pub summary: SegmentSummary,
}
