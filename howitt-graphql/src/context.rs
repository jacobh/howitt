use howitt::repos::{PointOfInterestRepo, RideModelRepo, RouteModelRepo};

use crate::credentials::Credentials;

pub struct SchemaData {
    pub poi_repo: PointOfInterestRepo,
    pub route_repo: RouteModelRepo,
    pub ride_repo: RideModelRepo,
}

pub struct RequestData {
    pub credentials: Option<Credentials>,
}
