mod cache;
mod simplified_ride_points;
mod simplified_route_points;
mod simplified_trip_elevation_points;

pub use simplified_ride_points::*;
pub use simplified_route_points::*;
pub use simplified_trip_elevation_points::*;

use super::simplify_points::SimplifyTarget;

#[derive(Debug)]
pub struct PointsFetcherParams {
    pub target: SimplifyTarget,
}
