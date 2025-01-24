use std::{collections::HashSet, error::Error, iter, marker::PhantomData};

use anyhow::anyhow;
use futures::prelude::*;
use itertools::Itertools;
use rwgps_types::{RouteSummary, TripSummary};

use crate::{
    ext::iter::ResultIterExt,
    models::{
        external_ref::{ExternalId, ExternalRef, ExternalRefItemMap, ExternalRefMatch, RwgpsId},
        photo::{Photo, PhotoId},
        point::{ElevationPoint, TemporalElevationPoint},
        ride::{Ride, RideId, RidePoints},
        route::{Route, RouteId, RouteModel},
        route_description::RouteDescription,
        tag::Tag,
        user::UserId,
        IndexItem,
    },
    repos::Repo,
    services::{
        simplify_points::{simplify_points, SimplifyTarget},
        smoothing::smooth_elevation_points,
    },
};

const SYNC_VERSION: usize = 2;

#[derive(Debug, Clone)]
pub struct SyncParams {
    pub rwgps_user_id: usize,
    pub user_id: UserId,
}

pub struct RwgpsSyncService<
    RouteRepo: Repo<Model = RouteModel>,
    RideRepo: Repo<Model = Ride>,
    RidePointsRepo: Repo<Model = RidePoints>,
    RwgpsClient: rwgps_types::client::RwgpsClient<Error = RwgpsClientError>,
    RwgpsClientError: Into<anyhow::Error>,
    ForceSyncRouteFn: Fn(&RouteSummary) -> bool,
> {
    pub route_repo: RouteRepo,
    pub ride_repo: RideRepo,
    pub ride_points_repo: RidePointsRepo,
    pub rwgps_client: RwgpsClient,
    pub rwgps_error: PhantomData<RwgpsClientError>,
    pub should_force_sync_route_fn: Option<ForceSyncRouteFn>,
}

impl<R1, R2, R3, C, E, F> RwgpsSyncService<R1, R2, R3, C, E, F>
where
    R1: Repo<Model = RouteModel>,
    R2: Repo<Model = Ride>,
    R3: Repo<Model = RidePoints>,
    C: rwgps_types::client::RwgpsClient<Error = E>,
    E: Error + Send + Sync + 'static,
    F: Fn(&RouteSummary) -> bool,
{
    pub fn new(
        route_repo: R1,
        ride_repo: R2,
        ride_points_repo: R3,
        rwgps_client: C,
        should_force_sync_route_fn: Option<F>,
    ) -> RwgpsSyncService<R1, R2, R3, C, E, F> {
        RwgpsSyncService {
            route_repo,
            ride_repo,
            ride_points_repo,
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
        SyncParams { rwgps_user_id, .. }: &SyncParams,
    ) -> Result<Vec<(RouteSummary, Option<Route>)>, anyhow::Error> {
        let existing_routes = self.route_repo.all_indexes().await?;
        let route_summaries = self.rwgps_client.user_routes(*rwgps_user_id).await?;

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
        SyncParams { rwgps_user_id, .. }: &SyncParams,
    ) -> Result<Vec<(TripSummary, Option<Ride>)>, anyhow::Error> {
        let existing_rides = self.ride_repo.all_indexes().await?;
        let trip_summaries = self.rwgps_client.user_trips(*rwgps_user_id).await?;

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
        SyncParams { user_id, .. }: &SyncParams,
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
            iter::empty()
                .chain(
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
                )
                .chain(description.as_ref().map_or(Vec::new(), |description| {
                    description
                        .tags
                        .clone()
                        .into_iter()
                        .map(Tag::Custom)
                        .collect_vec()
                })),
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

        let points = smooth_elevation_points(points);

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

        let name = route.name.replace("[BCS]", "").trim().to_string();

        let model = RouteModel::new(
            Route {
                id,
                name: name.clone(),
                slug: name.to_lowercase().replace(' ', "-"),
                user_id: user_id.clone(),
                distance: route.distance.unwrap_or(0.0),
                description,
                sample_points: Some(simplify_points(&points, SimplifyTarget::TotalPoints(50))),
                external_ref: Some(ExternalRef {
                    id: ExternalId::Rwgps(RwgpsId::Route(route.id)),
                    sync_version: Some(SYNC_VERSION),
                    updated_at: route.updated_at,
                }),
                tags,
            },
            points,
            photos,
        );

        self.route_repo.put(model).await?;

        Ok(())
    }

    async fn sync_ride(
        &self,
        trip_id: usize,
        existing_ride: Option<Ride>,
        SyncParams { user_id, .. }: &SyncParams,
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

        let ride = Ride {
            id,
            name: ride.name,
            user_id: *user_id,
            distance: ride.distance,
            started_at,
            finished_at,
            external_ref: Some(ExternalRef {
                id: ExternalId::Rwgps(RwgpsId::Trip(ride.id)),
                updated_at: ride.updated_at,
                sync_version: Some(SYNC_VERSION),
            }),
        };

        let ride_points = RidePoints { id, points };

        self.ride_repo.put(ride).await?;
        self.ride_points_repo.put(ride_points).await?;

        Ok(())
    }

    pub async fn sync(&self, params: SyncParams) -> Result<(), anyhow::Error> {
        let route_sync_candidates = self.detect_route_sync_candidates(&params).await?;
        let ride_sync_candidates = self.detect_ride_sync_candidates(&params).await?;

        dbg!(route_sync_candidates.len());
        dbg!(ride_sync_candidates.len());

        let futures = route_sync_candidates
            .into_iter()
            .map(|candidate| (candidate, self, params.clone()))
            .map(
                async move |((summary, existing_route), sync_service, params)| {
                    sync_service
                        .sync_route(summary.id, existing_route, &params)
                        .await
                },
            );

        let stream = stream::iter(futures);

        let results = stream.buffered(10).collect::<Vec<_>>().await;

        dbg!(results.len());

        results.into_iter().collect_result_vec()?;

        let futures = ride_sync_candidates
            .into_iter()
            .map(|candidate| (candidate, self, params.clone()))
            .map(
                async move |((summary, existing_route), sync_service, params)| {
                    sync_service
                        .sync_ride(summary.id, existing_route, &params)
                        .await
                },
            );

        let stream = stream::iter(futures);

        let results = stream.buffered(10).collect::<Vec<_>>().await;

        dbg!(results.len());

        results.into_iter().collect_result_vec()?;

        Ok(())
    }
}
