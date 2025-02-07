use rwgps_types::client::AuthenticatedRwgpsClient;

use crate::{
    models::{
        external_ref::{ExternalId, ExternalRef, ExternalRefItemMap, ExternalRefMatch, RwgpsId},
        route::RouteFilter,
        user::UserRwgpsConnection,
    },
    repos::RouteRepo,
};

pub struct SyncRouteHistoryParams<RwgpsClient> {
    pub client: RwgpsClient,
    pub route_repo: RouteRepo,
    pub connection: UserRwgpsConnection,
}

pub struct RouteSyncCandidate {
    pub rwgps_route_id: usize,
}

pub async fn select_historical_route_sync_candidates<
    RwgpsClient: rwgps_types::client::RwgpsClient,
>(
    SyncRouteHistoryParams {
        client,
        connection,
        route_repo,
    }: SyncRouteHistoryParams<RwgpsClient>,
) -> Result<Vec<RouteSyncCandidate>, anyhow::Error> {
    let existing_routes = route_repo
        .filter_models(RouteFilter::UserId(connection.user_id))
        .await?;

    let client = client.with_credentials(rwgps_types::credentials::Credentials::from_token(
        connection.access_token,
    ));

    let route_summaries = client
        .user_routes(connection.rwgps_user_id as usize)
        .await?;

    let existing_routes = ExternalRefItemMap::from_externally_reffed(existing_routes);

    Ok(route_summaries
        .into_iter()
        .filter_map(|summary| {
            match existing_routes.match_ref(ExternalRef {
                id: ExternalId::Rwgps(RwgpsId::Route(summary.id)),
                updated_at: summary.updated_at,
                sync_version: Some(2),
            }) {
                ExternalRefMatch::Fresh(_) => None,
                ExternalRefMatch::Stale(_) | ExternalRefMatch::NotFound => {
                    Some(RouteSyncCandidate {
                        rwgps_route_id: summary.id,
                    })
                }
            }
        })
        .collect())
}
