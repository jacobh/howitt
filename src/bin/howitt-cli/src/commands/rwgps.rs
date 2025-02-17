use std::sync::Arc;

use crate::Context;
use clap::{arg, Args, Subcommand};
use howitt::jobs::rwgps::RwgpsJob;
use howitt::jobs::Job;
use howitt::services::sync::rwgps_v2::select_historical_route_sync_candidates::{
    select_historical_route_sync_candidates, SyncRouteHistoryParams,
};
use howitt::services::sync::rwgps_v2::select_historical_trip_sync_candidates::{
    select_historical_trip_sync_candidates, SyncTripHistoryParams,
};
use howitt::{models::user::UserId, repos::AnyhowRepo};
use howitt_postgresql::PostgresRepos;
use rwgps_types::{client::RwgpsClient, credentials::Credentials};

#[derive(Subcommand)]
pub enum RwgpsCommands {
    Info(InfoArgs),
    EnqHistorySync(EnqHistorySync),
    HistorySyncDryRun(HistorySyncDryRun),
}

#[derive(Args)]
pub struct InfoArgs {
    #[arg(long)]
    user_id: String,
}

#[derive(Args)]
pub struct EnqHistorySync {
    #[arg(long)]
    user_id: String,
}

#[derive(Args)]
pub struct HistorySyncDryRun {
    #[arg(long)]
    user_id: String,
}

pub async fn handle(
    command: &RwgpsCommands,
    Context {
        repos:
            PostgresRepos {
                user_repo,
                route_repo,
                ride_repo,
                ..
            },
        job_storage,
        ..
    }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        RwgpsCommands::Info(InfoArgs { user_id }) => {
            let user_id = UserId::from(uuid::Uuid::parse_str(user_id)?);

            // Fetch user from repo
            let user = user_repo.get(user_id).await?;

            // Get RWGPS connection details
            let rwgps_connection = user
                .rwgps_connection
                .ok_or_else(|| anyhow::anyhow!("User has no RWGPS connection"))?;

            // Create RWGPS client
            let rwgps_client = rwgps::RwgpsClient::new();
            let auth_client = rwgps_client
                .with_credentials(Credentials::from_token(rwgps_connection.access_token));

            // Fetch user info
            let user_info = auth_client.user_info().await?;

            println!("RWGPS User Info for {}", user.username);
            dbg!(user_info);

            // Fetch routes count
            let routes = auth_client
                .user_routes(rwgps_connection.rwgps_user_id as usize)
                .await?;
            println!("Found {} routes", routes.len());

            // Fetch trips count
            let trips = auth_client
                .user_trips(rwgps_connection.rwgps_user_id as usize)
                .await?;
            println!("Found {} trips", trips.len());
        }
        RwgpsCommands::EnqHistorySync(EnqHistorySync { user_id }) => {
            let user_id = UserId::from(uuid::Uuid::parse_str(user_id)?);

            // Fetch user from repo
            let user = user_repo.get(user_id).await?;

            // Get RWGPS connection
            let rwgps_connection = user
                .rwgps_connection
                .ok_or_else(|| anyhow::anyhow!("User has no RWGPS connection"))?;

            // Enqueue the sync job
            job_storage
                .push(Job::from(RwgpsJob::SyncHistory {
                    connection: rwgps_connection,
                }))
                .await?;

            println!(
                "Successfully enqueued RWGPS history sync job for user {}",
                user.username
            );
        }
        RwgpsCommands::HistorySyncDryRun(HistorySyncDryRun { user_id }) => {
            let user_id = UserId::from(uuid::Uuid::parse_str(user_id)?);

            // Get user
            let user = user_repo.get(user_id).await?;

            // Get RWGPS connection
            let connection = user
                .rwgps_connection
                .ok_or_else(|| anyhow::anyhow!("No RWGPS connection found"))?;

            // Create RWGPS client
            let client = rwgps::RwgpsClient::new();

            // Check trip sync candidates
            let trip_candidates = select_historical_trip_sync_candidates(SyncTripHistoryParams {
                client: client.clone(),
                ride_repo: Arc::new(ride_repo.clone()),
                connection: connection.clone(),
            })
            .await?;

            println!("Found {} trip sync candidates:", trip_candidates.len());

            // Check route sync candidates
            let route_candidates =
                select_historical_route_sync_candidates(SyncRouteHistoryParams {
                    client: client.clone(),
                    route_repo: Arc::new(route_repo.clone()),
                    connection: connection.clone(),
                })
                .await?;

            println!("\nFound {} route sync candidates:", route_candidates.len());

            for candidate in &trip_candidates[..trip_candidates.len().min(3)] {
                println!("- Trip ID: {}", candidate.rwgps_trip_id);

                // Fetch and display trip details
                let auth_client =
                    client.with_credentials(rwgps_types::credentials::Credentials::from_token(
                        connection.access_token.clone(),
                    ));

                match auth_client.trip(candidate.rwgps_trip_id).await {
                    Ok(trip) => println!(
                        "  Name: {}\n  Distance: {:.2}km\n  Date: {}",
                        trip.name,
                        trip.distance / 1000.0,
                        trip.departed_at
                    ),
                    Err(e) => println!("  Failed to fetch trip details: {}", e),
                }
            }

            for candidate in &route_candidates[..route_candidates.len().min(3)] {
                println!("- Route ID: {}", candidate.rwgps_route_id);

                // Fetch and display route details
                let auth_client =
                    client.with_credentials(rwgps_types::credentials::Credentials::from_token(
                        connection.access_token.clone(),
                    ));

                match auth_client.route(candidate.rwgps_route_id).await {
                    Ok(route) => println!(
                        "  Name: {}\n  Distance: {:.2}km\n  Updated: {}",
                        route.name,
                        route.distance.unwrap_or(0.0) / 1000.0,
                        route.updated_at
                    ),
                    Err(e) => println!("  Failed to fetch route details: {}", e),
                }
            }
        }
    }

    Ok(())
}
