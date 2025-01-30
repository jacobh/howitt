use derive_more::derive::{From, Into};
use serde::{Deserialize, Serialize};

use crate::models::media::MediaId;

#[derive(Debug, Deserialize, Serialize, From, Into)]
pub struct ProcessMedia {
    pub media_id: MediaId,
}

#[derive(Debug, Deserialize, Serialize, From)]
pub enum MediaJob {
    Process(ProcessMedia),
}
