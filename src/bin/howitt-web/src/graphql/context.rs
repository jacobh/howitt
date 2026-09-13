use crate::{jobs::DynJobQueue, timezone::TimezoneLookup};
use async_graphql::dataloader::{DataLoader, HashMapCache};
use howitt::{
    repos::Repos,
    services::{
        fetchers::{SimplifiedRidePointsFetcher, SimplifiedTripElevationPointsFetcher},
        user::auth::Login,
    },
};

use super::loaders::{
    ride_loader::RideLoader,
    route_data_loader::RouteDataLoader,
    trip_content_loader::{TripMediaLoader, TripRidesLoader},
    user_loader::UserLoader,
};

pub struct SchemaData {
    pub repos: Repos,
    pub simplified_ride_points_fetcher: SimplifiedRidePointsFetcher<crate::cache::WorkerCache>,
    pub simplified_trip_elevation_points_fetcher:
        SimplifiedTripElevationPointsFetcher<crate::cache::WorkerCache>,
    pub ride_loader: DataLoader<RideLoader, HashMapCache>,
    pub user_loader: DataLoader<UserLoader, HashMapCache>,
    pub trip_rides_loader: DataLoader<TripRidesLoader, HashMapCache>,
    pub trip_media_loader: DataLoader<TripMediaLoader, HashMapCache>,
    pub route_points_loader: DataLoader<RouteDataLoader<crate::cache::WorkerCache>, HashMapCache>,
    pub tz_finder: TimezoneLookup,
    pub jobs: DynJobQueue,
    pub rwgps_client_id: String,
    pub rwgps_redirect_uri: String,
    pub user_auth_service: howitt::services::user::auth::UserAuthService,
}

pub struct RequestData {
    pub login: Option<Login>,
}
