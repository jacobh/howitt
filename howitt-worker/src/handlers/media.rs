use apalis::prelude::*;
use tracing::info;

use howitt::jobs::media::MediaJob;

pub async fn handle_media_job(job: MediaJob) -> Result<(), Error> {
    info!("Handling job: {:?}", job);
    Ok(())
}
