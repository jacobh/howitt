use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Tag {
    BackcountrySegment,
    Published {
        published_at: chrono::DateTime<chrono::Utc>,
    },
    Custom(String),
}
