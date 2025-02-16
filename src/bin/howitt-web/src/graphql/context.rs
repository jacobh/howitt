use async_graphql::dataloader::DataLoader;
use howitt::{
    jobs::Job,
    repos::Repos,
    services::{
        fetchers::{SimplifiedRidePointsFetcher, SimplifiedTripElevationPointsFetcher},
        user::auth::{Login, UserAuthService},
    },
};
use howitt_clients::RedisClient;
use howitt_jobs::storage::LockFreeStorage;
use tzf_rs::DefaultFinder;

use super::loaders::{
    ride_loader::RideLoader, route_points_loader::RoutePointsLoader, user_loader::UserLoader,
};

pub struct SchemaData {
    pub repos: Repos,
    pub simplified_ride_points_fetcher: SimplifiedRidePointsFetcher<RedisClient>,
    pub simplified_trip_elevation_points_fetcher: SimplifiedTripElevationPointsFetcher<RedisClient>,
    pub ride_loader: DataLoader<RideLoader>,
    pub user_loader: DataLoader<UserLoader>,
    pub route_points_loader: DataLoader<RoutePointsLoader>,
    pub rwgps_client_id: String,
    pub user_auth_service: UserAuthService,
    pub job_storage: LockFreeStorage<Job>,
    pub tz_finder: DefaultFinder,
}

pub struct RequestData {
    pub login: Option<Login>,
}
