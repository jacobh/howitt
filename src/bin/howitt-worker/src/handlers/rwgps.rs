use howitt::jobs::rwgps::RwgpsJob;
use thiserror::Error;

use crate::context::Context;

#[derive(Debug, Error)]
pub enum RwgpsJobError {}

pub async fn handle_rwgps_job(job: RwgpsJob, _ctx: Context) -> Result<(), RwgpsJobError> {
    dbg!(job);
    Ok(())
}
