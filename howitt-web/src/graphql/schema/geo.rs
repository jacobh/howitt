use async_graphql::{Enum, SimpleObject};

#[derive(SimpleObject)]
struct Point {
    lat: f64,
    lng: f64,
}

impl From<geo::Point<f64>> for Point {
    fn from(value: geo::Point<f64>) -> Self {
        Point {
            lat: value.y(),
            lng: value.x(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::cardinal_direction::CardinalDirection")]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(SimpleObject)]
pub struct PointDelta {
    pub distance: f64,
    pub bearing: f64,
    pub elevation_gain: f64,
}

impl From<howitt::models::point::ElevationPointDelta> for PointDelta {
    fn from(
        howitt::models::point::PointDelta {
            distance,
            bearing,
            data,
        }: howitt::models::point::ElevationPointDelta,
    ) -> Self {
        PointDelta {
            distance,
            bearing,
            elevation_gain: data.elevation_gain,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::slope_end::SlopeEnd")]
pub enum SlopeEnd {
    Uphill,
    Downhill,
    Flat,
}
