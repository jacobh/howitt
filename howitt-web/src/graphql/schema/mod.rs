use async_graphql::{Object, scalar};
use derive_more::derive::From;
use howitt::models::{
    photo::PhotoId, point_of_interest::PointOfInterestId, ride::RideId, route::RouteId,
    user::UserId,
};
use serde::{Deserialize, Serialize};

pub mod cue;
pub mod geo;
pub mod photo;
pub mod point_of_interest;
pub mod query;
pub mod ride;
pub mod route;
pub mod user;
pub mod viewer;

#[derive(Serialize, Deserialize, From)]
pub struct ModelId<ID: howitt::models::ModelId>(ID);

scalar!(ModelId<PointOfInterestId>, "PointOfInterestId");
scalar!(ModelId<RideId>, "RideId");
scalar!(ModelId<RouteId>, "RouteId");
scalar!(ModelId<PhotoId>, "PhotoId");
scalar!(ModelId<UserId>, "UserId");

pub struct ExternalRef(howitt::models::external_ref::ExternalRef);

#[Object]
impl ExternalRef {
    async fn canonical_url(&self) -> url::Url {
        self.0.id.canonical_url()
    }
}
