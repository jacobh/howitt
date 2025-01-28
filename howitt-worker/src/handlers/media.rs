use std::sync::Arc;

use apalis::prelude::*;
use tracing::info;

use howitt::{
    jobs::media::{MediaJob, ProcessMedia},
    repos::Repo,
};
use howitt_client_types::BucketClient;

use crate::context::Context;

pub async fn handle_media_job(job: MediaJob, ctx: Data<Context>) -> Result<(), Error> {
    info!("Handling job: {:?}", job);

    match job {
        MediaJob::Process(ProcessMedia { media_id }) => {
            let media = ctx
                .media_repo
                .get(media_id)
                .await
                .map_err(|e| Error::Failed(Arc::new(Box::new(e))))?;

            let bytes = ctx
                .bucket_client
                .get_object(&media.path)
                .await
                .map_err(|e| Error::Failed(Arc::new(Box::new(e))))?
                .ok_or_else(|| {
                    Error::MissingData(format!("Media object not found in bucket: {}", media.path))
                })?;

            let size_kb = bytes.len() as f64 / 1024.0;
            info!("Media file size: {:.2} KB", size_kb);

            Ok(())
        }
    }
}
