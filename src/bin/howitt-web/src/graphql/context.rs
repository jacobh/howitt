use crate::timezone::TimezoneLookup;
use async_graphql::dataloader::DataLoader;
use howitt::{
    repos::Repos,
    services::{
        fetchers::{SimplifiedRidePointsFetcher, SimplifiedTripElevationPointsFetcher},
        user::auth::Login,
    },
};

use super::loaders::{
    ride_loader::RideLoader, route_points_loader::RoutePointsLoader, user_loader::UserLoader,
};

pub struct SchemaData {
    pub repos: Repos,
    pub simplified_ride_points_fetcher: SimplifiedRidePointsFetcher<crate::cache::Uncached>,
    pub simplified_trip_elevation_points_fetcher:
        SimplifiedTripElevationPointsFetcher<crate::cache::Uncached>,
    pub ride_loader: DataLoader<RideLoader>,
    pub user_loader: DataLoader<UserLoader>,
    pub route_points_loader: DataLoader<RoutePointsLoader>,
    pub tz_finder: TimezoneLookup,
}

pub struct RequestData {
    pub login: Option<Login>,
}
