use async_graphql::Interface;

use crate::graphql::schema::{
    media::Media, point_of_interest::PointOfInterest, ride::Ride, route::Route, trip::Trip,
};

#[derive(Interface)]
#[graphql(field(name = "media", ty = "Vec<Media>"))]
pub enum MediaTarget {
    Ride(Ride),
    Route(Route),
    Trip(Trip),
    PointOfInterest(PointOfInterest),
}
