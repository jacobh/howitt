use std::{collections::HashSet, error::Error, marker::PhantomData};

use anyhow::anyhow;
use either::Either;
use itertools::Itertools;
use rwgps_types::{RouteSummary, TripSummary};

use crate::{
    ext::futures::FuturesIteratorExt,
    ext::iter::ResultIterExt,
    models::{
        config::{Config, ConfigId},
        external_ref::{ExternalId, ExternalRef, ExternalRefItemMap, ExternalRefMatch, RwgpsId},
        photo::{Photo, PhotoId},
        point::{simplify_points, ElevationPoint, PointChunk, TemporalElevationPoint},
        ride::{Ride, RideId, RideModel},
        route::{Route, RouteId, RouteModel},
        route_description::RouteDescription,
        tag::Tag,
        terminus::Termini,
        IndexItem,
    },
    repos::Repo,
};

const SYNC_VERSION: usize = 2;

pub struct RwgpsSyncService<
    RouteRepo: Repo<Model = RouteModel>,
    RideRepo: Repo<Model = RideModel>,
    ConfigRepo: Repo<Model = Config>,
    RwgpsClient: rwgps_types::client::RwgpsClient<Error = RwgpsClientError>,
    RwgpsClientError: Into<anyhow::Error>,
    ForceSyncRouteFn: Fn(&RouteSummary) -> bool,
> {
    pub route_repo: RouteRepo,
    pub ride_repo: RideRepo,
    pub config_repo: ConfigRepo,
    pub rwgps_client: RwgpsClient,
    pub rwgps_error: PhantomData<RwgpsClientError>,
    pub should_force_sync_route_fn: Option<ForceSyncRouteFn>,
}

impl<R1, R2, R3, C, E, F> RwgpsSyncService<R1, R2, R3, C, E, F>
where
    R1: Repo<Model = RouteModel>,
    R2: Repo<Model = RideModel>,
    R3: Repo<Model = Config>,
    C: rwgps_types::client::RwgpsClient<Error = E>,
    E: Error + Send + Sync + 'static,
    F: Fn(&RouteSummary) -> bool,
{
    pub fn new(
        route_repo: R1,
        ride_repo: R2,
        config_repo: R3,
        rwgps_client: C,
        should_force_sync_route_fn: Option<F>,
    ) -> RwgpsSyncService<R1, R2, R3, C, E, F> {
        RwgpsSyncService {
            route_repo,
            ride_repo,
            config_repo,
            rwgps_client,
            rwgps_error: PhantomData,
            should_force_sync_route_fn,
        }
    }

    fn should_force_sync_route(&self, summary: &RouteSummary) -> bool {
        match &self.should_force_sync_route_fn {
            Some(f) => f(summary),
            None => false,
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
                    id: ExternalId::Rwgps(RwgpsId::Route(summary.id)),
                    updated_at: summary.updated_at,
                    sync_version: Some(SYNC_VERSION),
                }) {
                    ExternalRefMatch::Fresh(route) => {
                        if self.should_force_sync_route(&summary) {
                            Some((summary, Some(route.clone())))
                        } else {
                            None
                        }
                    }
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
                    id: ExternalId::Rwgps(RwgpsId::Trip(summary.id)),
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
        let existing_route_model = match &existing_route {
            Some(route) => Some(self.route_repo.get(route.id()).await?),
            None => None,
        };
        let existing_photos = existing_route_model
            .as_ref()
            .map(|route| route.photos.clone())
            .unwrap_or_default();

        let route = self.rwgps_client.route(route_id).await?;

        let id = RouteId::get_or_from_datetime(
            existing_route.map(|route| route.id()),
            &route.created_at,
        );

        let description = route
            .description
            .map(RouteDescription::parse)
            .transpose()?
            .flatten();

        let tags: HashSet<Tag> = HashSet::from_iter(
            [
                if route.name.contains("[BCS]") {
                    Some(Tag::BackcountrySegment)
                } else {
                    None
                },
                if let Some(description) = description.as_ref() {
                    description
                        .published_at
                        .map(|published_at| Tag::Published { published_at })
                } else {
                    None
                },
            ]
            .into_iter()
            .flatten(),
        );

        let points = route
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
            .map(|(point, elevation)| ElevationPoint { point, elevation })
            .collect_vec();

        let photos = route
            .photos
            .into_iter()
            .map(|photo| {
                let external_ref = ExternalRef {
                    id: ExternalId::Rwgps(RwgpsId::Photo(photo.id)),
                    updated_at: photo.updated_at,
                    sync_version: Some(SYNC_VERSION),
                };

                let existing_photo = existing_photos.iter().find(|existing_photo| {
                    existing_photo.external_ref.as_ref().map(|r| &r.id) == Some(&external_ref.id)
                });

                match existing_photo {
                    Some(existing_photo) => Photo {
                        caption: photo.caption,
                        ..existing_photo.clone()
                    },
                    None => Photo {
                        model_id: id,
                        id: PhotoId::from_datetime(photo.created_at),
                        url: external_ref.id.canonical_url(),
                        external_ref: Some(external_ref),
                        caption: photo.caption,
                    },
                }
            })
            .collect_vec();

        let model = RouteModel::new(
            Route {
                id: Either::Right(id),
                name: route.name.replace("[BCS]", "").trim().to_string(),
                distance: route.distance.unwrap_or(0.0),
                description,
                sample_points: Some(simplify_points(&points, 50)),
                external_ref: Some(ExternalRef {
                    id: ExternalId::Rwgps(RwgpsId::Route(route.id)),
                    sync_version: Some(SYNC_VERSION),
                    updated_at: route.updated_at,
                }),
                tags,
            },
            PointChunk::new_chunks(id, points.into_iter()),
            photos,
        );

        self.route_repo.put(model).await?;

        Ok(())
    }

    async fn sync_ride(
        &self,
        trip_id: usize,
        existing_ride: Option<Ride>,
    ) -> Result<(), anyhow::Error> {
        let ride = self.rwgps_client.trip(trip_id).await?;

        let id = RideId::get_or_from_datetime(
            existing_ride.map(|ride| ride.model_id()),
            &ride.created_at,
        );

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
                    id: ExternalId::Rwgps(RwgpsId::Trip(ride.id)),
                    updated_at: ride.updated_at,
                    sync_version: Some(SYNC_VERSION),
                }),
            },
            point_chunks: PointChunk::new_chunks(id, points),
        };

        self.ride_repo.put(model).await?;

        Ok(())
    }

    async fn sync_starred_routes(&self) -> Result<Vec<RouteId>, anyhow::Error> {
        let routes = self.route_repo.all_indexes().await?;

        let starred_route_ids: Vec<RouteId> = routes
            .into_iter()
            .filter(|route| route.tags.contains(&Tag::BackcountrySegment))
            .map(|route| route.model_id())
            .collect();

        let mut config = self.config_repo.get(ConfigId).await?;
        config.starred_route_ids = starred_route_ids.clone();
        self.config_repo.put(config).await?;

        Ok(starred_route_ids)
    }

    pub async fn sync(&self, rwgps_user_id: usize) -> Result<(), anyhow::Error> {
        let route_sync_candidates = self.detect_route_sync_candidates(rwgps_user_id).await?;
        let ride_sync_candidates = self.detect_ride_sync_candidates(rwgps_user_id).await?;

        dbg!(route_sync_candidates.len());
        dbg!(ride_sync_candidates.len());

        let results = route_sync_candidates
            .into_iter()
            .map(|candidate| (candidate, self))
            .map(async move |((summary, existing_route), sync_service)| {
                sync_service.sync_route(summary.id, existing_route).await
            })
            .collect_futures_ordered()
            .await;

        dbg!(results.len());

        results.into_iter().collect_result_vec()?;

        let results = ride_sync_candidates
            .into_iter()
            .map(|candidate| (candidate, self))
            .map(async move |((summary, existing_route), sync_service)| {
                sync_service.sync_ride(summary.id, existing_route).await
            })
            .collect_futures_ordered()
            .await;

        dbg!(results.len());

        results.into_iter().collect_result_vec()?;

        dbg!(self.sync_starred_routes().await?.len());

        Ok(())
    }
}
