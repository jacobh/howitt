use chrono::{DateTime, Utc};

use super::{ride::RideId, user::UserId, ModelName, ModelUlid};

pub type TripId = ModelUlid<{ ModelName::Trip }>;

pub struct Trip {
    pub id: TripId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub user_id: UserId,
    pub ride_ids: Vec<RideId>,
}
