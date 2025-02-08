use clap::Subcommand;
use howitt::{
    jobs::{media::MediaJob, Job},
    models::media::MediaFilter,
    repos::Repo,
};
use howitt_postgresql::PostgresRepos;

use crate::Context;

#[derive(Subcommand)]
pub enum MediaCommands {
    ProcessAll,
}

pub async fn handle(
    command: &MediaCommands,
    Context {
        repos: PostgresRepos { media_repo, .. },
        job_storage,
        ..
    }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        MediaCommands::ProcessAll => {
            let media = media_repo.filter_models(MediaFilter::All).await?;

            for media_item in media.clone() {
                job_storage
                    .push(Job::from(MediaJob::Process(media_item.id)))
                    .await?;

                println!("Enqueued processing job for media {}", media_item.id);
            }

            println!("Enqueued {} media items for processing", media.len());
            Ok(())
        }
    }
}
