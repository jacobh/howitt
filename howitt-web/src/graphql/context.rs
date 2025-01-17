use howitt::{
    repos::{PointOfInterestRepo, RideRepo, RouteModelRepo, UserRepo},
    services::user::auth::Login,
};

pub struct SchemaData {
    pub poi_repo: PointOfInterestRepo,
    pub route_repo: RouteModelRepo,
    pub ride_repo: RideRepo,
    pub user_repo: UserRepo,
}

pub struct RequestData {
    pub login: Option<Login>,
}
