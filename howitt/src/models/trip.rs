use super::{ride::RideId, user::UserId, IndexModel, ModelName, ModelUlid};
use chrono::{DateTime, Utc};

pub type TripId = ModelUlid<{ ModelName::Trip }>;

#[derive(Debug, Clone)]
pub struct Trip {
    pub id: TripId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub user_id: UserId,
    pub ride_ids: Vec<RideId>,
}

#[derive(Debug, Clone)]
pub enum TripFilter {
    All,
    User(UserId),
}

impl IndexModel for Trip {
    type Id = TripId;
    type Filter = TripFilter;

    fn id(&self) -> Self::Id {
        self.id
    }
}
