use thiserror::Error;

use crate::{
    models::user::UserRwgpsConnection,
    repos::{RoutePointsRepo, RouteRepo},
};

#[derive(Debug, Error)]
pub enum RwgpsSyncError {}

pub struct SyncRouteParams<RwgpsClient> {
    pub client: RwgpsClient,
    pub rwgps_route_id: usize,
    pub connection: UserRwgpsConnection,
    pub route_repo: RouteRepo,
    pub route_points_repo: RoutePointsRepo,
}

pub fn sync_route<RwgpsClient: rwgps_types::client::AuthenticatedRwgpsClient>(
    SyncRouteParams {
        client,
        rwgps_route_id,
        connection,
        route_repo,
        route_points_repo,
    }: SyncRouteParams<RwgpsClient>,
) -> Result<(), RwgpsSyncError> {
    Ok(())
}
