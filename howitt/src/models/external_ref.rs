use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExternalSource {
    Rwgps,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalRef {
    pub id: String,
    pub source: ExternalSource,
    pub updated_at: DateTime<Utc>,
    pub sync_version: Option<usize>,
}
