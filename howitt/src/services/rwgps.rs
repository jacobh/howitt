use std::{error::Error, marker::PhantomData};

use itertools::Itertools;
use rwgps_types::RouteSummary;

use crate::{
    ext::futures::FuturesIteratorExt,
    ext::iter::ResultIterExt,
    models::{
        external_ref::{ExternalRef, ExternalSource},
        point::{ElevationPoint, PointChunk},
        ride::RideModel,
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

        Ok(route_summaries
            .into_iter()
            .filter_map(|summary| {
                let existing_route = existing_routes
                    .iter()
                    .filter_map(|route| {
                        route
                            .external_ref
                            .as_ref()
                            .map(|external_ref| (route, external_ref))
                    })
                    .find(|(_, external_ref)| {
                        external_ref.source == ExternalSource::Rwgps
                            && Some(SYNC_VERSION) == external_ref.sync_version
                            && external_ref.id == summary.id.to_string()
                    });

                match existing_route {
                    Some((route, external_ref)) => {
                        if &&summary.updated_at > &&external_ref.updated_at {
                            Some((summary, Some(route.clone())))
                        } else {
                            None
                        }
                    }
                    None => Some((summary, None)),
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

    pub async fn sync(&self, rwgps_user_id: usize) -> Result<(), anyhow::Error> {
        let route_sync_candidates = self.detect_route_sync_candidates(rwgps_user_id).await?;

        dbg!(&route_sync_candidates);

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

        Ok(())
    }
}
