use clap::Subcommand;
use howitt_fs::{load_huts, load_localities, load_stations};
use howitt_postgresql::PostgresPointOfInterestRepo;

#[derive(Subcommand)]
pub enum POICommands {
    Sync,
    List,
    Stations,
    Huts,
    Localities,
}

pub async fn handle(command: &POICommands) -> Result<(), anyhow::Error> {
    match command {
        POICommands::Sync => {
            // TODO: Implement sync functionality
            Ok(())
        }
        POICommands::List => {
            // TODO: Implement list functionality
            Ok(())
        }
        POICommands::Stations => {
            let railway_stations = load_stations()?;
            dbg!(railway_stations.len());
            Ok(())
        }
        POICommands::Huts => {
            let huts = load_huts()?;
            dbg!(huts);
            Ok(())
        }
        POICommands::Localities => {
            load_localities()?;
            Ok(())
        }
    }
}
