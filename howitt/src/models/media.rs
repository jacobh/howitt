use chrono::{DateTime, Utc};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};

use super::{
    point_of_interest::PointOfInterestId, ride::RideId, route::RouteId, trip::TripId, user::UserId,
    IndexModel, ModelName, ModelUuid,
};

pub type MediaId = ModelUuid<{ ModelName::Media }>;

#[derive(Debug, Clone, From, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaRelationId {
    Ride(RideId),
    Route(RouteId),
    Trip(TripId),
    PointOfInterest(PointOfInterestId),
}
impl MediaRelationId {
    pub fn as_ride_id(&self) -> Option<RideId> {
        match self {
            Self::Ride(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_route_id(&self) -> Option<RouteId> {
        match self {
            Self::Route(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_trip_id(&self) -> Option<TripId> {
        match self {
            Self::Trip(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_point_of_interest_id(&self) -> Option<PointOfInterestId> {
        match self {
            Self::PointOfInterest(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Media {
    pub id: MediaId,
    pub created_at: DateTime<Utc>,
    pub user_id: UserId,
    pub path: String,
    pub relation_ids: Vec<MediaRelationId>,
}

impl Media {
    pub fn iter_ride_ids(&self) -> impl Iterator<Item = RideId> + '_ {
        self.relation_ids
            .iter()
            .filter_map(|relation| relation.as_ride_id())
    }

    pub fn iter_route_ids(&self) -> impl Iterator<Item = RouteId> + '_ {
        self.relation_ids
            .iter()
            .filter_map(|relation| relation.as_route_id())
    }

    pub fn iter_trip_ids(&self) -> impl Iterator<Item = TripId> + '_ {
        self.relation_ids
            .iter()
            .filter_map(|relation| relation.as_trip_id())
    }

    pub fn iter_point_of_interest_ids(&self) -> impl Iterator<Item = PointOfInterestId> + '_ {
        self.relation_ids
            .iter()
            .filter_map(|relation| relation.as_point_of_interest_id())
    }
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
