#![feature(async_closure)]

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use commands::{POICommands, RideCommands, RouteCommands, UserCommands};

mod commands;
mod postgres;
mod utils;

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
    Pg(crate::postgres::Postgres),
    #[clap(subcommand)]
    User(UserCommands),
    #[clap(subcommand)]
    Route(RouteCommands),
    #[clap(subcommand)]
    Ride(RideCommands),
    #[clap(subcommand)]
    POI(POICommands),
}

#[derive(Args)]
struct GpxInfo {
    filepath: PathBuf,
}

#[derive(Args)]
struct Stations {
    ptv_gtfs_dirpath: PathBuf,
}

#[derive(Args)]
struct Huts {
    filepath: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::User(cmd) => commands::user::handle(cmd).await?,
        Commands::Route(cmd) => commands::route::handle(cmd).await?,
        Commands::Ride(cmd) => commands::ride::handle(cmd).await?,
        Commands::POI(cmd) => commands::poi::handle(cmd).await?,
        Commands::Rwgps(command) => commands::rwgps::handle(command).await?,
        Commands::Pg(command) => crate::postgres::handle(command).await?,
    }

    Ok(())
}
