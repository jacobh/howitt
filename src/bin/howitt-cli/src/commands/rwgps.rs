use crate::Context;
use clap::{arg, Args, Subcommand};
use howitt::jobs::rwgps::RwgpsJob;
use howitt::jobs::Job;
use howitt::{models::user::UserId, repos::AnyhowRepo};
use howitt_fs::load_routes;
use howitt_fs::load_user_config;
use howitt_postgresql::PostgresRepos;
use itertools::Itertools;
use rwgps_types::{
    client::RwgpsClient,
    config::UserConfig,
    credentials::{Credentials, PasswordCredentials},
};

#[derive(Subcommand)]
pub enum RwgpsCommands {
    Info(InfoArgs),
    #[clap(subcommand)]
    Routes(Routes),
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

#[derive(Subcommand)]
pub enum Routes {
    List,
    Detail(RouteDetailArgs),
}

#[derive(Args)]
pub struct RouteDetailArgs {
    route_id: usize,
}

fn get_user_config() -> Result<UserConfig, anyhow::Error> {
    let config = load_user_config()?;

    match config {
        Some(config) => Ok(config),
        None => {
            println!("Initial user setup");
            let email = inquire::Text::new("Email").prompt();
            let password = inquire::Password::new("Password")
                .without_confirmation()
                .prompt();

            match (email, password) {
                (Ok(email), Ok(password)) => Ok(UserConfig {
                    password_info: PasswordCredentials { email, password },
                    user_info: None,
                }),
                _ => anyhow::bail!("Invalid email/password"),
            }
        }
    }
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
        RwgpsCommands::Routes(Routes::List) => {
            let routes: Vec<rwgps_types::Route> = load_routes()?
                .into_iter()
                .sorted_by_key(|route| route.id)
                .collect_vec();

            let rows = vec![prettytable::row![
                "id",
                "name",
                "distance (km)",
                "last modified"
            ]]
            .into_iter()
            .chain(routes.into_iter().map(|route| {
                prettytable::row![
                    route.id,
                    route.name,
                    route.distance.unwrap_or(0.0) / 1000.0,
                    route.updated_at,
                ]
            }))
            .collect_vec();

            prettytable::Table::init(rows).printstd()
        }
        RwgpsCommands::Routes(Routes::Detail(args)) => {
            let user_config = get_user_config()?;
            let client = rwgps::AuthenticatedRwgpsClient::new(user_config.credentials());

            let resp = client.route(args.route_id).await?;
            dbg!(resp);
        }
        RwgpsCommands::Trips => {
            unimplemented!()
        }
    }

    Ok(())
}
