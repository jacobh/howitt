use apalis::prelude::*;

use howitt::jobs::Job;

mod media;

pub async fn handle_job(job: Job) -> Result<(), Error> {
    match job {
        Job::Media(media_job) => media::handle_media_job(media_job).await,
    }
}
