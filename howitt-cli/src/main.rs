#![feature(async_closure)]

use clap::{Parser, Subcommand};
use commands::{POICommands, RideCommands, RouteCommands, UserCommands};

mod commands;
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
    User(UserCommands),
    #[clap(subcommand)]
    Route(RouteCommands),
    #[clap(subcommand)]
    Ride(RideCommands),
    #[clap(subcommand)]
    POI(POICommands),
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
    }

    Ok(())
}
