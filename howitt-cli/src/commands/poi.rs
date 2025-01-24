use clap::Subcommand;
use howitt::repos::Repo;
use howitt_fs::{load_huts, load_localities, load_stations};
use howitt_postgresql::{PostgresClient, PostgresPointOfInterestRepo};

#[derive(Subcommand)]
pub enum POICommands {
    Sync,
    List,
    Stations,
    Huts,
    Localities,
}

pub async fn handle(command: &POICommands) -> Result<(), anyhow::Error> {
    let pg = PostgresClient::connect(
        &std::env::var("DATABASE_URL")
            .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
    )
    .await?;

    let point_of_interest_repo = PostgresPointOfInterestRepo::new(pg.clone());

    match command {
        POICommands::Sync => {
            let stations = load_stations()?;
            let huts = load_huts()?;

            point_of_interest_repo.put_batch(stations).await?;
            point_of_interest_repo.put_batch(huts).await?;

            println!("done");
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
