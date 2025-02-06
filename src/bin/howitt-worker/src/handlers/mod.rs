use std::sync::Arc;

use apalis::prelude::*;

use howitt::jobs::Job;

use crate::context::Context;

mod media;
mod rwgps;

pub async fn handle_job(job: Job, ctx: Data<Context>) -> Result<(), Error> {
    match job {
        Job::Media(media_job) => media::handle_media_job(media_job, ctx)
            .await
            .map_err(|e| Error::Failed(Arc::new(Box::new(e)))),
        Job::Rwgps(rwgps_job) => rwgps::handle_rwgps_job(rwgps_job, ctx)
            .await
            .map_err(|e| Error::Failed(Arc::new(Box::new(e)))),
    }
}
