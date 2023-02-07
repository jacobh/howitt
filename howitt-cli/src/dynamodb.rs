use clap::Subcommand;
use futures::{prelude::*, stream::FuturesUnordered};
use howitt_dynamo::{CheckpointRepo, SingleTableClient, DynamoRepo};
use howitt_fs::load_huts;

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
            let huts = load_huts()?;

            let x: Vec<Result<_, _>> = huts
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
