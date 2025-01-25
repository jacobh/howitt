use async_graphql::{scalar, Object};
use derive_more::derive::From;
use howitt::models::{
    photo::PhotoId, point_of_interest::PointOfInterestId, ride::RideId, route::RouteId,
    trip::TripId, user::UserId,
};
use interfaces::elevation_data::ElevationData;
use serde::{Deserialize, Serialize};

use super::context::SchemaData;

pub mod interfaces;
pub mod objects;
pub mod scalars;

pub use objects::*;

#[derive(Serialize, Deserialize, From)]
pub struct ModelId<ID: howitt::models::ModelId>(ID);

scalar!(ModelId<PhotoId>, "PhotoId");
scalar!(ModelId<PointOfInterestId>, "PointOfInterestId");
scalar!(ModelId<RideId>, "RideId");
scalar!(ModelId<RouteId>, "RouteId");
scalar!(ModelId<TripId>, "TripId");
scalar!(ModelId<UserId>, "UserId");

pub struct ExternalRef(howitt::models::external_ref::ExternalRef);

#[Object]
impl ExternalRef {
    async fn canonical_url(&self) -> url::Url {
        self.0.id.canonical_url()
    }
}

pub type Schema = async_graphql::Schema<
    query::Query,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
>;

pub fn build_schema(data: SchemaData) -> Schema {
    Schema::build(
        query::Query,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .register_output_type::<ElevationData>()
    .data(data)
    .finish()
}
