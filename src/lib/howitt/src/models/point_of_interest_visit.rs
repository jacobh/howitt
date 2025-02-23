use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{media::MediaId, point_of_interest::PointOfInterestId, user::UserId};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum VisitConfirmation {
    Pending,
    Confirmed,
    Rejected,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum POICondition {
    AllGood,
    Issue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfInterestVisit {
    pub user_id: UserId,
    pub point_of_interest_id: PointOfInterestId,
    pub visited_at: DateTime<Utc>,
    pub confirmation: VisitConfirmation,
    pub condition: Option<POICondition>,
    pub comment: Option<String>,
    pub media_ids: Vec<MediaId>,
}
