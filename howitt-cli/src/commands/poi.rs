use clap::Subcommand;
use howitt_fs::{load_huts, load_stations};
use howitt_postgresql::PostgresPointOfInterestRepo;

#[derive(Subcommand)]
pub enum POICommands {
    Sync,
    List,
    Stations,
    Huts,
}

pub async fn handle(command: &POICommands) -> Result<(), anyhow::Error> {
    match command {
        _ => Ok(()), // Placeholder - implement actual handlers
    }
}
