use crate::Context;
use clap::{arg, Args, Subcommand};
use howitt::jobs::rwgps::RwgpsJob;
use howitt::jobs::Job;
use howitt::{models::user::UserId, repos::AnyhowRepo};
use howitt_postgresql::PostgresRepos;
use rwgps_types::{client::RwgpsClient, credentials::Credentials};

#[derive(Subcommand)]
pub enum RwgpsCommands {
    Info(InfoArgs),
    Trips,
    EnqHistorySync(EnqHistorySync),
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
pub struct RouteDetailArgs {
    route_id: usize,
}

pub async fn handle(
    command: &RwgpsCommands,
    Context {
        repos: PostgresRepos { user_repo, .. },
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
        RwgpsCommands::Trips => {
            unimplemented!()
        }
    }

    Ok(())
}
