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
    ProcessExtension {
        #[clap(name = "extension")]
        extension: String,
    },
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
        MediaCommands::ProcessExtension { extension } => {
            let media = media_repo.filter_models(MediaFilter::All).await?;
            let extension = extension.to_lowercase();

            println!("Scanning media items for .{} files...", extension);

            let mut processed_count = 0;
            for media_item in media.clone() {
                // Extract the file extension from the path
                if let Some(file_ext) = media_item
                    .path
                    .split('.')
                    .last()
                    .map(|ext| ext.to_lowercase())
                {
                    if file_ext == extension {
                        job_storage
                            .push(Job::from(MediaJob::Process(media_item.id)))
                            .await?;

                        println!(
                            "Enqueued processing job for media {} ({})",
                            media_item.id, media_item.path
                        );
                        processed_count += 1;
                    }
                }
            }

            println!(
                "Found {} media items with .{} extension, enqueued {} for processing",
                media.len(),
                extension,
                processed_count
            );
            Ok(())
        }
    }
}
