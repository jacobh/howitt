use apalis::prelude::*;

use howitt::jobs::Job;

use crate::context::Context;

mod media;

pub async fn handle_job(job: Job, ctx: Data<Context>) -> Result<(), Error> {
    match job {
        Job::Media(media_job) => media::handle_media_job(media_job, ctx).await,
    }
}
