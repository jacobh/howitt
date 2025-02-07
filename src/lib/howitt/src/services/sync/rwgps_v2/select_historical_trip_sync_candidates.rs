use rwgps_types::client::AuthenticatedRwgpsClient;

use crate::{
    models::{
        external_ref::{ExternalId, ExternalRef, ExternalRefItemMap, ExternalRefMatch, RwgpsId},
        ride::RideFilter,
        user::UserRwgpsConnection,
    },
    repos::RideRepo,
};

pub struct SyncTripHistoryParams<RwgpsClient> {
    pub client: RwgpsClient,
    pub ride_repo: RideRepo,
    pub connection: UserRwgpsConnection,
}

pub struct TripSyncCandidate {
    pub rwgps_trip_id: usize,
}

pub async fn select_historical_trip_sync_candidates<
    RwgpsClient: rwgps_types::client::RwgpsClient,
>(
    SyncTripHistoryParams {
        client,
        connection,
        ride_repo,
    }: SyncTripHistoryParams<RwgpsClient>,
) -> Result<Vec<TripSyncCandidate>, anyhow::Error> {
    let existing_rides = ride_repo
        .filter_models(RideFilter::ForUser {
            user_id: connection.user_id,
            started_at: None,
        })
        .await?;

    let client = client.with_credentials(rwgps_types::credentials::Credentials::from_token(
        connection.access_token,
    ));

    let trip_summaries = client.user_trips(connection.rwgps_user_id as usize).await?;

    let existing_rides = ExternalRefItemMap::from_externally_reffed(existing_rides);

    Ok(trip_summaries
        .into_iter()
        .filter_map(|summary| {
            match existing_rides.match_ref(ExternalRef {
                id: ExternalId::Rwgps(RwgpsId::Trip(summary.id)),
                updated_at: summary.updated_at,
                sync_version: Some(2),
            }) {
                ExternalRefMatch::Fresh(_) => None,
                ExternalRefMatch::Stale(_) | ExternalRefMatch::NotFound => {
                    Some(TripSyncCandidate {
                        rwgps_trip_id: summary.id,
                    })
                }
            }
        })
        .collect())
}
