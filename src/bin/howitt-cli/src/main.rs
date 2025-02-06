#![feature(async_closure)]

use std::sync::Arc;

use apalis_redis::RedisStorage;
use clap::{Parser, Subcommand};
use howitt::jobs::Job;
use howitt_postgresql::{PostgresClient, PostgresRepos};

mod commands;
mod utils;

use commands::{
    MediaCommands, POICommands, RideCommands, RouteCommands, TripCommands, UserCommands,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(subcommand)]
    Rwgps(commands::RwgpsCommands),
    #[clap(subcommand)]
    User(UserCommands),
    #[clap(subcommand)]
    Route(RouteCommands),
    #[clap(subcommand)]
    Ride(RideCommands),
    #[clap(subcommand)]
    POI(POICommands),
    #[clap(subcommand)]
    Trip(TripCommands),
    #[clap(subcommand)]
    Media(MediaCommands),
    OnceOff,
}

pub struct Context {
    pub postgres_client: PostgresClient,
    pub repos: PostgresRepos,
    pub job_storage: Arc<tokio::sync::Mutex<RedisStorage<Job>>>,
}

impl Context {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let postgres_client = PostgresClient::connect(
            &std::env::var("DATABASE_URL")
                .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
        )
        .await?;

        let conn = apalis_redis::connect(
            std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1:6379/")),
        )
        .await?;

        let job_storage = RedisStorage::new(conn);

        Ok(Self {
            repos: PostgresRepos::new(postgres_client.clone()),
            postgres_client,
            job_storage: Arc::new(tokio::sync::Mutex::new(job_storage)),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let context = Context::new().await?;

    match &cli.command {
        Commands::User(cmd) => commands::user::handle(cmd, context).await?,
        Commands::Route(cmd) => commands::route::handle(cmd, context).await?,
        Commands::Ride(cmd) => commands::ride::handle(cmd, context).await?,
        Commands::POI(cmd) => commands::poi::handle(cmd, context).await?,
        Commands::Rwgps(cmd) => commands::rwgps::handle(cmd, context).await?,
        Commands::Trip(cmd) => commands::trip::handle(cmd, context).await?,
        Commands::Media(cmd) => commands::media::handle(cmd, context).await?,
        Commands::OnceOff => commands::once_off::handle(context).await?,
    }

    Ok(())
}
