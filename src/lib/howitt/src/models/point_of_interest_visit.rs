use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{media::MediaId, point_of_interest::PointOfInterestId, user::UserId};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum VisitStatus {
    AllGood,
    Issue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfInterestVisit {
    pub user_id: UserId,
    pub point_of_interest_id: PointOfInterestId,
    pub visited_at: DateTime<Utc>,
    pub status: VisitStatus,
    pub comment: Option<String>,
    pub media_ids: Vec<MediaId>,
}
