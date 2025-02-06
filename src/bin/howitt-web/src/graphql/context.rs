use async_graphql::dataloader::DataLoader;
use howitt::{
    repos::{MediaRepo, PointOfInterestRepo, RideRepo, RouteRepo, TripRepo, UserRepo},
    services::{
        fetchers::SimplifiedRidePointsFetcher,
        user::auth::{Login, UserAuthService},
    },
};
use howitt_clients::RedisClient;

use super::loaders::{route_points_loader::RoutePointsLoader, user_loader::UserLoader};

pub struct SchemaData {
    pub poi_repo: PointOfInterestRepo,
    pub route_repo: RouteRepo,
    pub ride_repo: RideRepo,
    pub trip_repo: TripRepo,
    pub user_repo: UserRepo,
    pub simplified_ride_points_fetcher: SimplifiedRidePointsFetcher<RedisClient>,
    pub user_loader: DataLoader<UserLoader>,
    pub route_points_loader: DataLoader<RoutePointsLoader>,
    pub media_repo: MediaRepo,
    pub rwgps_client_id: String,
    pub user_auth_service: UserAuthService,
}

pub struct RequestData {
    pub login: Option<Login>,
}
