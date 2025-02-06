use crate::repos::{RidePointsRepo, RideRepo, RoutePointsRepo, RouteRepo};

pub struct RwgpsSyncContext {
    pub ride_repo: RideRepo,
    pub ride_points_repo: RidePointsRepo,
    pub route_repo: RouteRepo,
    pub route_points_repo: RoutePointsRepo,
}
