use clap::Subcommand;
use howitt::{models::trip::TripFilter, repos::Repo};
use prettytable::{row, Table};

use crate::Context;

#[derive(Subcommand)]
pub enum TripCommands {
    List,
    // You can add more commands here later like Create, Detail, etc.
}

pub async fn handle(
    command: &TripCommands,
    Context { trip_repo, .. }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        TripCommands::List => {
            let trips = trip_repo.filter_models(TripFilter::All).await?;

            let mut table = Table::new();
            table.add_row(row!["id", "name", "year", "rides"]);

            for trip in trips {
                table.add_row(row![trip.id, trip.name, trip.year, trip.ride_ids.len()]);
            }

            table.printstd();
            Ok(())
        }
    }
}
