use std::{error::Error, marker::PhantomData};

use crate::{
    ext::futures::FuturesIteratorExt,
    models::{ride::RideModel, route::RouteModel},
    repos::Repo,
};

pub struct RwgpsSyncService<
    RouteRepo: Repo<RouteModel>,
    RideRepo: Repo<RideModel>,
    RwgpsClient: rwgps_types::client::RwgpsClient<Error = RwgpsClientError>,
    RwgpsClientError: Into<anyhow::Error>,
> {
    pub route_repo: RouteRepo,
    pub ride_repo: RideRepo,
    pub rwgps_client: RwgpsClient,
    pub rwgps_error: PhantomData<RwgpsClientError>,
}

impl<R1, R2, C, E> RwgpsSyncService<R1, R2, C, E>
where
    R1: Repo<RouteModel>,
    R2: Repo<RideModel>,
    C: rwgps_types::client::RwgpsClient<Error = E>,
    E: Error + Send + Sync + 'static,
{
    pub fn new(route_repo: R1, ride_repo: R2, rwgps_client: C) -> RwgpsSyncService<R1, R2, C, E> {
        RwgpsSyncService {
            route_repo,
            ride_repo,
            rwgps_client,
            rwgps_error: PhantomData,
        }
    }

    async fn fetch_data(
        &self,
        rwgps_user_id: usize,
    ) -> Result<(Vec<rwgps_types::Route>, Vec<rwgps_types::Trip>), anyhow::Error> {
        let client = self.rwgps_client.clone();

        let route_summaries = self.rwgps_client.user_routes(rwgps_user_id).await?;

        let routes: Vec<Result<rwgps_types::Route, _>> = route_summaries
            .into_iter()
            .map(|route| (route, client.clone()))
            .map(async move |(route, client)| client.route(route.id).await)
            .collect_futures_ordered()
            .await;

        let routes = routes.into_iter().collect::<Result<Vec<_>, _>>()?;

        // persist_routes(&routes)?;
        dbg!(routes.len());

        let trip_summaries = client.user_trips(rwgps_user_id).await?;

        let trips: Vec<Result<rwgps_types::Trip, _>> = trip_summaries
            .into_iter()
            .map(|trip| (trip, client.clone()))
            .map(async move |(trip, client)| client.trip(trip.id).await)
            .collect_futures_ordered()
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

    pub async fn sync(&self, rwgps_user_id: usize) -> Result<(), anyhow::Error> {
        let (routes, trips) = self.fetch_data(rwgps_user_id).await?;
        self.persist_routes(routes).await?;
        self.persist_trips(trips).await?;

        Ok(())
    }
}
