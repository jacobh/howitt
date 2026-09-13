use crate::Context;
use clap::Subcommand;
use howitt::{
    jobs::{media::MediaJob, Job},
    models::media::MediaId,
};

#[derive(Subcommand)]
pub enum MediaCommands {
    /// Infer location from existing rides. Image transformations are a separate task.
    InferLocation {
        #[arg(long)]
        media_id: uuid::Uuid,
    },
}

pub async fn handle(command: &MediaCommands, _context: Context) -> anyhow::Result<()> {
    match command {
        MediaCommands::InferLocation { media_id } => {
            howitt_jobs::enqueue(Job::Media(MediaJob::InferLocation(MediaId::from(
                *media_id,
            ))))
            .await?;
            println!("Enqueued location inference for media {media_id}");
        }
    }
    Ok(())
}
