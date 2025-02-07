use serde::{Deserialize, Serialize};

use crate::models::media::MediaId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MediaJob {
    Process(MediaId),
    InferLocation(MediaId),
}
