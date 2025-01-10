use howitt::{
    repos::{PointOfInterestRepo, RideModelRepo, RouteModelRepo},
    services::user::auth::Login,
};

pub struct SchemaData {
    pub poi_repo: PointOfInterestRepo,
    pub route_repo: RouteModelRepo,
    pub ride_repo: RideModelRepo,
}

pub struct RequestData {
    pub login: Option<Login>,
}
