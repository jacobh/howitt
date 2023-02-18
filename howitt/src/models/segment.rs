use serde::{Deserialize, Serialize};

use crate::{models::checkpoint::Checkpoint};

crate::model_id!(SegmentId, "SEGMENT");

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: SegmentId,
    pub start: Checkpoint,
    pub end: Checkpoint,
    pub route: gpx::Route,
}
