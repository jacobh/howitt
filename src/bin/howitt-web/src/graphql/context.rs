use async_graphql::dataloader::DataLoader;
use howitt::{
    jobs::Job,
    repos::Repos,
    services::{
        fetchers::SimplifiedRidePointsFetcher,
        user::auth::{Login, UserAuthService},
    },
};
use howitt_clients::RedisClient;
use howitt_jobs::storage::LockFreeStorage;

use super::loaders::{route_points_loader::RoutePointsLoader, user_loader::UserLoader};

pub struct SchemaData {
    pub repos: Repos,
    pub simplified_ride_points_fetcher: SimplifiedRidePointsFetcher<RedisClient>,
    pub user_loader: DataLoader<UserLoader>,
    pub route_points_loader: DataLoader<RoutePointsLoader>,
    pub rwgps_client_id: String,
    pub user_auth_service: UserAuthService,
    pub job_storage: LockFreeStorage<Job>,
}

pub struct RequestData {
    pub login: Option<Login>,
}
