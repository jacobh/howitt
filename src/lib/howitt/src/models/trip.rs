use super::{media::MediaId, ride::RideId, user::UserId, Model, ModelName, ModelUuid};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub notes: Vec<TripNote>,
    pub ride_ids: Vec<RideId>,
    pub media_ids: Vec<MediaId>,
}

#[derive(Debug, Clone)]
pub enum TripFilter {
    All,
    User(UserId),
    WithUserAndSlug { user_id: UserId, slug: String },
}

impl Model for Trip {
    type Id = TripId;
    type Filter = TripFilter;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripNote {
    pub timestamp: DateTime<Utc>,
    pub text: String,
}
