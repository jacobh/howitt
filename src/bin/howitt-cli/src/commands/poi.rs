use clap::Subcommand;
use howitt::repos::Repo;
use howitt_fs::{load_huts, load_localities, load_stations};

use crate::Context;

#[derive(Subcommand)]
pub enum POICommands {
    Sync,
    List,
    Stations,
    Huts,
    Localities,
}

pub async fn handle(
    command: &POICommands,
    Context { poi_repo, .. }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        POICommands::Sync => {
            let stations = load_stations()?;
            let huts = load_huts()?;

            poi_repo.put_batch(stations).await?;
            poi_repo.put_batch(huts).await?;

            println!("done");
            Ok(())
        }
        POICommands::List => {
            let pois = poi_repo.all_indexes().await?;
            dbg!(pois);
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
