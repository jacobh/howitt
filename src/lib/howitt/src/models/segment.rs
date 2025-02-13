use serde::{Deserialize, Serialize};

use crate::models::point_of_interest::PointOfInterest;

use super::{ModelName, ModelUuid};

pub type SegmentId = ModelUuid<{ ModelName::Segment }>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: SegmentId,
    pub start: PointOfInterest,
    pub end: PointOfInterest,
    pub route: gpx::Route,
}
