use chrono::{DateTime, Utc};

use super::{
    point_of_interest::PointOfInterestId, ride::RideId, route::RouteId, trip::TripId, user::UserId,
    IndexModel, ModelName, ModelUuid,
};

pub type MediaId = ModelUuid<{ ModelName::Media }>;

#[derive(Debug, Clone)]
pub struct Media {
    pub id: MediaId,
    pub created_at: DateTime<Utc>,
    pub user_id: UserId,
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum MediaFilter {
    All,
    ForUser(UserId),
    ForRide(RideId),
    ForRoute(RouteId),
    ForTrip(TripId),
    ForPointOfInterest(PointOfInterestId),
}

impl IndexModel for Media {
    type Id = MediaId;
    type Filter = MediaFilter;

    fn id(&self) -> Self::Id {
        self.id
    }
}
