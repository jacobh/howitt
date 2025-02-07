use apalis::prelude::*;
use howitt::jobs::rwgps::RwgpsJob;
use howitt::jobs::Job;
use howitt::models::user::UserFilter;
use howitt::repos::Repos;
use howitt::services::sync::rwgps_v2::sync_route::{sync_route, SyncRouteParams};
use howitt::services::sync::rwgps_v2::sync_trip::{sync_trip, SyncTripParams};
use rwgps_types::webhook::ItemType;
use thiserror::Error;
use tracing;

use crate::context::Context;

#[derive(Debug, Error)]
pub enum RwgpsJobError {
    #[error("Failed to process RWGPS job: {0}")]
    Processing(#[from] anyhow::Error),
}

pub async fn handle_rwgps_job(
    job: RwgpsJob,
    Context {
        repos:
            Repos {
                user_repo,
                route_repo,
                route_points_repo,
                ride_repo,
                ride_points_repo,
                ..
            },
        rwgps_client,
        job_storage,
        ..
    }: Context,
) -> Result<(), RwgpsJobError> {
    match job {
        RwgpsJob::Webhook(notification) => {
            tracing::info!(
                    item_type = ?notification.item_type,
                    item_id = notification.item_id,
            user_id = notification.user_id,
                    action = ?notification.action,
                    "Processing RWGPS {:?} webhook", notification.item_type
                );

            tracing::info!(
                user_id = notification.user_id,
                "Looking up user by RWGPS ID"
            );

            // Get user from repo
            let user = user_repo
                .find_model(UserFilter::RwgpsId(notification.user_id as usize))
                .await?
                .ok_or(RwgpsJobError::Processing(anyhow::anyhow!(
                    "No user found with RWGPS ID"
                )))?;
            tracing::info!(
                user_id = notification.user_id,
                howitt_user_id = user.id.to_string(),
                "Found user, checking for RWGPS connection"
            );

            // Get RWGPS connection
            let connection = user
                .rwgps_connection
                .ok_or_else(|| anyhow::anyhow!("User has no RWGPS connection"))?;

            tracing::debug!(
                user_id = notification.user_id,
                howitt_user_id = user.id.to_string(),
                "Found RWGPS connection"
            );

            match notification.item_type {
                ItemType::Route => {
                    job_storage
                        .lock()
                        .await
                        .push(Job::Rwgps(RwgpsJob::SyncRoute {
                            rwgps_route_id: notification.item_id as usize,
                            connection,
                        }))
                        .await
                        .map_err(|e| RwgpsJobError::Processing(e.into()))?;

                    tracing::info!(
                        route_id = notification.item_id,
                        "Successfully processed RWGPS route webhook"
                    );
                }
                ItemType::Trip => {
                    job_storage
                        .lock()
                        .await
                        .push(Job::Rwgps(RwgpsJob::SyncTrip {
                            rwgps_trip_id: notification.item_id as usize,
                            connection,
                        }))
                        .await
                        .map_err(|e| RwgpsJobError::Processing(e.into()))?;

                    tracing::info!(
                        trip_id = notification.item_id,
                        "Successfully processed RWGPS trip webhook"
                    );
                }
            }
        }
        RwgpsJob::SyncRoute {
            rwgps_route_id,
            connection,
        } => {
            tracing::info!(route_id = rwgps_route_id, "Processing RWGPS route sync");

            // Sync the route
            sync_route(SyncRouteParams {
                client: rwgps_client,
                route_repo,
                route_points_repo,
                rwgps_route_id,
                connection,
            })
            .await?;

            tracing::info!(
                route_id = rwgps_route_id,
                "Successfully processed RWGPS route sync"
            );
        }
        RwgpsJob::SyncTrip {
            rwgps_trip_id,
            connection,
        } => {
            tracing::info!(trip_id = rwgps_trip_id, "Processing RWGPS trip sync");

            // Sync the trip
            sync_trip(SyncTripParams {
                client: rwgps_client,
                ride_repo,
                ride_points_repo,
                rwgps_trip_id,
                connection,
            })
            .await?;

            tracing::info!(
                trip_id = rwgps_trip_id,
                "Successfully processed RWGPS trip sync"
            );
        }
    }

    Ok(())
}
