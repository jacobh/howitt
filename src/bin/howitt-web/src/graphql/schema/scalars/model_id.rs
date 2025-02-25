use async_graphql::scalar;
use derive_more::derive::From;
use howitt::models::{
    media::MediaId, note::NoteId, point_of_interest::PointOfInterestId, ride::RideId,
    route::RouteId, trip::TripId, user::UserId,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, From)]
pub struct ModelId<ID: howitt::models::ModelId>(pub ID);

scalar!(ModelId<MediaId>, "MediaId");
scalar!(ModelId<PointOfInterestId>, "PointOfInterestId");
scalar!(ModelId<RideId>, "RideId");
scalar!(ModelId<RouteId>, "RouteId");
scalar!(ModelId<TripId>, "TripId");
scalar!(ModelId<UserId>, "UserId");
scalar!(ModelId<NoteId>, "NoteId");
