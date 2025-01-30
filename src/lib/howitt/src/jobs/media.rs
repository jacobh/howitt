use serde::{Deserialize, Serialize};

use crate::models::media::MediaId;

#[derive(Debug, Deserialize, Serialize)]
pub enum MediaJob {
    Process(MediaId),
    InferLocation(MediaId),
}
