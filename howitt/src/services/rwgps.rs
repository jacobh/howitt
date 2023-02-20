use std::{error::Error, marker::PhantomData};

use futures::{prelude::*, stream::FuturesUnordered};

use crate::repos::{RouteModelRepo, RideModelRepo};

pub struct RwgpsSyncService<
    RwgpsClient: rwgps_types::client::RwgpsClient<RwgpsClientError>,
    RwgpsClientError: Error,
> {
    route_repo: RouteModelRepo,
    ride_repo: RideModelRepo,
    rwgps_client: RwgpsClient,
    rwgps_error: PhantomData<RwgpsClientError>,
}

impl<C: rwgps_types::client::RwgpsClient<E>, E: Error + Send + Sync + 'static> RwgpsSyncService<C, E> {
    pub fn new(route_repo: RouteModelRepo, ride_repo: RideModelRepo, rwgps_client: C) -> RwgpsSyncService<C, E> {
        RwgpsSyncService { route_repo, ride_repo, rwgps_client, rwgps_error: PhantomData }
    }

    async fn fetch_data(&self, rwgps_user_id: usize) -> Result<(Vec<rwgps_types::Route>, Vec<rwgps_types::Trip>), anyhow::Error> {
        let client = self.rwgps_client.clone();

        let route_summaries = self.rwgps_client
            .user_routes(rwgps_user_id)
            .await?;

            let routes: Vec<Result<rwgps_types::Route, _>> = route_summaries
                .into_iter()
                .map(|route| (route, client.clone()))
                .map(async move |(route, client)| client.route(route.id).await)
                .collect::<FuturesUnordered<_>>()
                .collect()
                .await;

            let routes = routes.into_iter().collect::<Result<Vec<_>, _>>()?;

            // persist_routes(&routes)?;
            dbg!(routes.len());

            let trip_summaries = client
                .user_trips(rwgps_user_id)
                .await?;

            let trips: Vec<Result<rwgps_types::Trip, _>> = trip_summaries
                .into_iter()
                .map(|trip| (trip, client.clone()))
                .map(async move |(trip, client)| client.trip(trip.id).await)
                .collect::<FuturesUnordered<_>>()
                .collect()
                .await;

            let trips: Vec<rwgps_types::Trip> = trips.into_iter().collect::<Result<Vec<_>, _>>()?;

            // persist_trips(&trips)?;
            dbg!(trips.len());

            Ok((routes, trips))
    }

    async fn persist_routes(&self, routes: Vec<rwgps_types::Route>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }

    async fn persist_trips(&self, trips: Vec<rwgps_types::Trip>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }

    pub async fn sync(&self) -> Result<(), anyhow::Error> {
        let rwgps_user_id = 1;
        let (routes, trips) = self.fetch_data(rwgps_user_id).await?;
        self.persist_routes(routes).await?;
        self.persist_trips(trips).await?;

        Ok(())
    }
}