use clap::Subcommand;
use howitt::repos::Repo;
use howitt_postgresql::PostgresRepos;

use crate::Context;

#[derive(Subcommand)]
pub enum POICommands {
    List,
}

pub async fn handle(
    command: &POICommands,
    Context {
        repos: PostgresRepos {
            point_of_interest_repo,
            ..
        },
        ..
    }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        POICommands::List => {
            let pois = point_of_interest_repo.all().await?;
            dbg!(pois);
            Ok(())
        }
    }
}
