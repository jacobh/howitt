use howitt::jobs::rwgps::RwgpsJob;
use howitt::models::user::UserFilter;
use howitt::repos::Repos;
use howitt::services::sync::rwgps_v2::sync_route::{sync_route, SyncRouteParams};
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
                ..
            },
        rwgps_client,
        ..
    }: Context,
) -> Result<(), RwgpsJobError> {
    match job {
        RwgpsJob::Webhook(notification) => {
            match notification.item_type {
                ItemType::Route => {
                    tracing::info!(
                        route_id = notification.item_id,
                        user_id = notification.user_id,
                        action = ?notification.action,
                        "Processing RWGPS route webhook"
                    );

                    // Get user from repo
                    let user = user_repo
                        .find_model(UserFilter::RwgpsId(notification.user_id as usize))
                        .await?
                        .ok_or(RwgpsJobError::Processing(anyhow::anyhow!(
                            "No user found with RWGPS ID"
                        )))?;
                    // Get RWGPS connection
                    let connection = user
                        .rwgps_connection
                        .ok_or_else(|| anyhow::anyhow!("User has no RWGPS connection"))?;

                    // Sync the route
                    sync_route(SyncRouteParams {
                        client: rwgps_client,
                        route_repo: route_repo,
                        route_points_repo: route_points_repo,
                        rwgps_route_id: notification.item_id as usize,
                        connection,
                    })
                    .await?;

                    tracing::info!(
                        route_id = notification.item_id,
                        "Successfully processed RWGPS route webhook"
                    );
                }
                ItemType::Trip => {
                    unimplemented!()
                }
            }
        }
    }

    Ok(())
}
