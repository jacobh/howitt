use anyhow::Context;
use howitt::{
    jobs::{media::MediaJob, rwgps::RwgpsJob, Job},
    models::user::{UserFilter, UserId, UserRwgpsConnection},
    repos::Repos,
    services::{
        media::MediaGeoInferrer,
        sync::rwgps_v2::{
            persistence::DynRwgpsSyncStore,
            select_historical_route_sync_candidates::{
                select_historical_route_sync_candidates, SyncRouteHistoryParams,
            },
            select_historical_trip_sync_candidates::{
                select_historical_trip_sync_candidates, SyncTripHistoryParams,
            },
            sync_route::{sync_route, SyncRouteParams},
            sync_trip::{sync_trip, SyncTripParams},
        },
    },
};
use rwgps_types::{client::RwgpsClient, webhook::ItemType};

async fn connection(repos: &Repos, user_id: UserId) -> anyhow::Result<UserRwgpsConnection> {
    repos
        .user_repo
        .get(user_id)
        .await?
        .rwgps_connection
        .context("User has no RWGPS connection")
}

/// Return follow-up jobs; the caller must publish all of them before acknowledging
/// the parent. Retrying a partial fan-out is safe because sync writes are idempotent.
pub async fn handle_job<C: RwgpsClient>(
    job: Job,
    repos: Repos,
    client: C,
    store: DynRwgpsSyncStore,
) -> anyhow::Result<Vec<Job>> {
    match job {
        Job::Media(MediaJob::InferLocation(media_id)) => {
            let media = repos.media_repo.get(media_id).await?;
            MediaGeoInferrer::new(repos.media_repo, repos.ride_repo, repos.ride_points_repo)
                .infer_ride_and_point_and_save(&media)
                .await?;
        }
        Job::Rwgps(RwgpsJob::Webhook(notification)) => {
            let user = repos
                .user_repo
                .find_model(UserFilter::RwgpsId(notification.user_id as usize))
                .await?
                .context("No user found for RWGPS webhook")?;
            let job = match notification.item_type {
                ItemType::Route => RwgpsJob::SyncRoute {
                    rwgps_route_id: notification.item_id as usize,
                    user_id: user.id,
                },
                ItemType::Trip => RwgpsJob::SyncTrip {
                    rwgps_trip_id: notification.item_id as usize,
                    user_id: user.id,
                },
            };
            return Ok(vec![Job::Rwgps(job)]);
        }
        Job::Rwgps(RwgpsJob::SyncRoute {
            rwgps_route_id,
            user_id,
        }) => {
            let connection = connection(&repos, user_id).await?;
            sync_route(SyncRouteParams {
                client,
                route_repo: repos.route_repo,
                store,
                rwgps_route_id,
                connection,
            })
            .await?;
        }
        Job::Rwgps(RwgpsJob::SyncTrip {
            rwgps_trip_id,
            user_id,
        }) => {
            let connection = connection(&repos, user_id).await?;
            sync_trip(SyncTripParams {
                client,
                ride_repo: repos.ride_repo,
                store,
                rwgps_trip_id,
                connection,
            })
            .await?;
        }
        Job::Rwgps(RwgpsJob::SyncHistory { user_id }) => {
            let connection = connection(&repos, user_id).await?;
            let routes = select_historical_route_sync_candidates(SyncRouteHistoryParams {
                client: client.clone(),
                route_repo: repos.route_repo,
                connection: connection.clone(),
            })
            .await?;
            let trips = select_historical_trip_sync_candidates(SyncTripHistoryParams {
                client,
                ride_repo: repos.ride_repo,
                connection,
            })
            .await?;
            return Ok(routes
                .into_iter()
                .map(|route| {
                    Job::Rwgps(RwgpsJob::SyncRoute {
                        rwgps_route_id: route.rwgps_route_id,
                        user_id,
                    })
                })
                .chain(trips.into_iter().map(|trip| {
                    Job::Rwgps(RwgpsJob::SyncTrip {
                        rwgps_trip_id: trip.rwgps_trip_id,
                        user_id,
                    })
                }))
                .collect());
        }
    }
    Ok(vec![])
}
