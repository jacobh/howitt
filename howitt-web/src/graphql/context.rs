use async_graphql::dataloader::DataLoader;
use howitt::{
    repos::{PointOfInterestRepo, RideRepo, RouteModelRepo, TripRepo, UserRepo},
    services::{fetchers::SimplifiedRidePointsFetcher, user::auth::Login},
};
use howitt_clients::RedisClient;

use super::loaders::user_loader::UserLoader;

pub struct SchemaData {
    pub poi_repo: PointOfInterestRepo,
    pub route_repo: RouteModelRepo,
    pub ride_repo: RideRepo,
    pub trip_repo: TripRepo,
    pub user_repo: UserRepo,
    pub simplified_ride_points_fetcher: SimplifiedRidePointsFetcher<RedisClient>,
    pub user_loader: DataLoader<UserLoader>,
}

pub struct RequestData {
    pub login: Option<Login>,
}
