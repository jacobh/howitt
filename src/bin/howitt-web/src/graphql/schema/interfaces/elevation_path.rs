use async_graphql::Interface;

use crate::graphql::schema::{ride::Ride, route::Route, trip::TripLeg};

#[derive(Interface)]
#[graphql(
    field(
        name = "elevation_points",
        ty = "Vec<f64>",
        desc = "Array of elevation points"
    ),
    field(
        name = "distance_points",
        ty = "Vec<f64>",
        desc = "Array of distance points"
    ),
    field(
        name = "elevation_points_json",
        ty = "String",
        desc = "Array of elevation points"
    ),
    field(
        name = "distance_points_json",
        ty = "String",
        desc = "Array of distance points"
    )
)]
pub enum ElevationPath {
    Ride(Ride),
    Route(Route),
    TripLeg(TripLeg),
}
