use clap::Subcommand;
use futures::{prelude::*, stream::FuturesUnordered};
use howitt_dynamo::{CheckpointRepo, SingleTableClient};
use howitt_fs::load_stations;

#[derive(Subcommand)]
pub enum Dynamodb {
    Sync,
    ListCheckpoints,
}

pub async fn handle(command: &Dynamodb) -> Result<(), anyhow::Error> {
    let client = SingleTableClient::new_from_env().await;
    let checkpoint_repo = CheckpointRepo::new(client);
    
    match command {
        Dynamodb::Sync => {
            let stations = load_stations()?;

            let x: Vec<Result<_, _>> = stations
                .into_iter()
                .map(|checkpoint| (checkpoint, checkpoint_repo.clone()))
                .map(async move |(checkpoint, checkpoint_repo)| {
                    checkpoint_repo.put(checkpoint).await
                })
                .collect::<FuturesUnordered<_>>()
                .collect()
                .await;

            dbg!(x);
        }
        Dynamodb::ListCheckpoints => {
            let checkpoints = checkpoint_repo.all().await?;
            dbg!(checkpoints);
        }
    }

    Ok(())
}
