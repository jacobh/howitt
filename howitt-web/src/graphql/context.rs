use howitt::{
    repos::{PointOfInterestRepo, RidePointsRepo, RideRepo, RouteModelRepo, UserRepo},
    services::{fetchers::SimplifiedRidePointsFetcher, user::auth::Login},
};
use howitt_clients::RedisClient;

pub struct SchemaData {
    pub poi_repo: PointOfInterestRepo,
    pub route_repo: RouteModelRepo,
    pub ride_repo: RideRepo,
    pub ride_points_repo: RidePointsRepo,
    pub user_repo: UserRepo,
    pub simplified_ride_points_fetcher: SimplifiedRidePointsFetcher<RedisClient>,
}

pub struct RequestData {
    pub login: Option<Login>,
}
