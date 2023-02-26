use std::{error::Error, marker::PhantomData};

use anyhow::anyhow;
use chrono::Utc;
use itertools::Itertools;
use rwgps_types::{RouteSummary, TripSummary};

use crate::{
    ext::futures::FuturesIteratorExt,
    ext::iter::ResultIterExt,
    models::{
        external_ref::{ExternalRef, ExternalRefItemMap, ExternalRefMatch, ExternalSource},
        point::{ElevationPoint, PointChunk, TemporalElevationPoint},
        ride::{Ride, RideId, RideModel},
        route::{Route, RouteId, RouteModel},
    },
    repos::Repo,
};

const SYNC_VERSION: usize = 2;

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

    async fn detect_route_sync_candidates(
        &self,
        rwgps_user_id: usize,
    ) -> Result<Vec<(RouteSummary, Option<Route>)>, anyhow::Error> {
        let existing_routes = self.route_repo.all_indexes().await?;
        let route_summaries = self.rwgps_client.user_routes(rwgps_user_id).await?;

        let existing_routes = ExternalRefItemMap::from_externally_reffed(existing_routes);

        Ok(route_summaries
            .into_iter()
            .filter_map(|summary| {
                match existing_routes.match_ref(ExternalRef {
                    id: summary.id.to_string(),
                    source: ExternalSource::Rwgps,
                    updated_at: summary.updated_at,
                    sync_version: Some(SYNC_VERSION),
                }) {
                    ExternalRefMatch::Fresh(_) => None,
                    ExternalRefMatch::Stale(route) => Some((summary, Some(route.clone()))),
                    ExternalRefMatch::NotFound => Some((summary, None)),
                }
            })
            .collect_vec())
    }

    async fn detect_ride_sync_candidates(
        &self,
        rwgps_user_id: usize,
    ) -> Result<Vec<(TripSummary, Option<Ride>)>, anyhow::Error> {
        let existing_rides = self.ride_repo.all_indexes().await?;
        let trip_summaries = self.rwgps_client.user_trips(rwgps_user_id).await?;

        let existing_rides = ExternalRefItemMap::from_externally_reffed(existing_rides);

        Ok(trip_summaries
            .into_iter()
            .filter_map(|summary| {
                match existing_rides.match_ref(ExternalRef {
                    id: summary.id.to_string(),
                    source: ExternalSource::Rwgps,
                    updated_at: summary.updated_at,
                    sync_version: Some(SYNC_VERSION),
                }) {
                    ExternalRefMatch::Fresh(_) => None,
                    ExternalRefMatch::Stale(ride) => Some((summary, Some(ride.clone()))),
                    ExternalRefMatch::NotFound => Some((summary, None)),
                }
            })
            .collect_vec())
    }

    async fn sync_route(
        &self,
        route_id: usize,
        existing_route: Option<Route>,
    ) -> Result<(), anyhow::Error> {
        let route = self.rwgps_client.route(route_id).await?;

        let id = match existing_route {
            Some(route) => route.id,
            None => ulid::Ulid::from_datetime(route.created_at.into()),
        };

        let model = RouteModel {
            route: Route {
                id,
                name: route.name,
                distance: route.distance.unwrap_or(0.0),
                external_ref: Some(ExternalRef {
                    source: ExternalSource::Rwgps,
                    sync_version: Some(SYNC_VERSION),
                    id: route.id.to_string(),
                    updated_at: route.updated_at,
                }),
            },
            point_chunks: PointChunk::new_chunks(
                RouteId::from(id),
                route
                    .track_points
                    .into_iter()
                    .filter_map(|track_point| {
                        match (
                            geo::Point::try_from(track_point.clone()),
                            track_point.elevation,
                        ) {
                            (Ok(point), Some(elevation)) => Some((point, elevation)),
                            _ => None,
                        }
                    })
                    .map(|(point, elevation)| ElevationPoint { point, elevation }),
            ),
        };

        self.route_repo.put(model).await?;

        Ok(())
    }

    async fn sync_ride(
        &self,
        trip_id: usize,
        existing_ride: Option<Ride>,
    ) -> Result<(), anyhow::Error> {
        let ride = self.rwgps_client.trip(trip_id).await?;

        let id = match existing_ride {
            Some(ride) => ride.id,
            None => RideId::from(ulid::Ulid::from_datetime(ride.created_at.into())),
        };

        let points = ride
            .track_points
            .into_iter()
            .filter_map(|track_point| {
                match (
                    geo::Point::try_from(track_point.clone()),
                    track_point.elevation,
                    track_point.datetime,
                ) {
                    (Ok(point), Some(elevation), Some(datetime)) => Some(TemporalElevationPoint {
                        datetime,
                        point,
                        elevation,
                    }),
                    _ => None,
                }
            })
            .collect_vec();

        let started_at = points
            .iter()
            .map(|point| point.datetime)
            .min()
            .ok_or(anyhow!("no points"))?;

        let finished_at = points
            .iter()
            .map(|point| point.datetime)
            .max()
            .ok_or(anyhow!("no points"))?;

        let model = RideModel {
            ride: Ride {
                id,
                name: ride.name,
                distance: ride.distance,
                started_at,
                finished_at,
                external_ref: Some(ExternalRef {
                    id: ride.id.to_string(),
                    source: ExternalSource::Rwgps,
                    updated_at: ride.updated_at,
                    sync_version: Some(SYNC_VERSION),
                }),
            },
            point_chunks: PointChunk::new_chunks(id, points),
        };

        self.ride_repo.put(model).await?;

        Ok(())
    }

    pub async fn sync(&self, rwgps_user_id: usize) -> Result<(), anyhow::Error> {
        let route_sync_candidates = self.detect_route_sync_candidates(rwgps_user_id).await?;
        let ride_sync_candidates = self.detect_ride_sync_candidates(rwgps_user_id).await?;

        dbg!(&route_sync_candidates);
        dbg!(&ride_sync_candidates);

        let results = route_sync_candidates
            .into_iter()
            .map(|candidate| (candidate, self.clone()))
            .map(async move |((summary, existing_route), sync_service)| {
                sync_service.sync_route(summary.id, existing_route).await
            })
            .collect_futures_ordered()
            .await;

        dbg!(&results);

        results.into_iter().collect_result_vec()?;

        let results = ride_sync_candidates
            .into_iter()
            .map(|candidate| (candidate, self.clone()))
            .map(async move |((summary, existing_route), sync_service)| {
                sync_service.sync_ride(summary.id, existing_route).await
            })
            .collect_futures_ordered()
            .await;

        dbg!(&results);

        results.into_iter().collect_result_vec()?;

        Ok(())
    }
}
