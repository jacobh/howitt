use super::{media::MediaId, ride::RideId, user::UserId, IndexModel, ModelName, ModelUuid};
use chrono::{DateTime, Utc};

pub type TripId = ModelUuid<{ ModelName::Trip }>;

#[derive(Debug, Clone)]
pub struct Trip {
    pub id: TripId,
    pub created_at: DateTime<Utc>,
    pub user_id: UserId,
    pub name: String,
    pub slug: String,
    pub year: i32,
    pub description: Option<String>,
    pub ride_ids: Vec<RideId>,
    pub media_ids: Vec<MediaId>,
}

#[derive(Debug, Clone)]
pub enum TripFilter {
    All,
    User(UserId),
    WithUserAndSlug { user_id: UserId, slug: String },
}

impl IndexModel for Trip {
    type Id = TripId;
    type Filter = TripFilter;

    fn id(&self) -> Self::Id {
        self.id
    }
}
