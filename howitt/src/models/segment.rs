use serde::{Deserialize, Serialize};

use crate::models::point_of_interest::PointOfInterest;

crate::model_id!(SegmentId, "SEGMENT");

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: SegmentId,
    pub start: PointOfInterest,
    pub end: PointOfInterest,
    pub route: gpx::Route,
}
