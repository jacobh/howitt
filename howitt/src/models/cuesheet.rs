use serde::{Deserialize, Serialize};

use super::{point_of_interest::PointOfInterest, segment_summary::SegmentElevationSummary};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cuesheet {
    pub cues: Vec<Cue>,
}

#[derive(Debug, Serialize, Deserialize, derive_more::Display)]
pub enum CueStop {
    #[display(fmt = "Start")]
    Start,
    #[display(fmt = "End")]
    End,
    #[display(fmt = "{} ({})", "_0.name", "_0.point_of_interest_type")]
    POI(PointOfInterest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cue {
    pub origin: CueStop,
    pub destination: CueStop,
    pub summary: SegmentElevationSummary,
}
