use async_graphql::Object;
use chrono::{DateTime, Utc};

use super::ride::Ride;

pub struct Note {
    pub content_at: DateTime<Utc>,
    pub text: String,
    pub ride: Option<Ride>,
}

#[Object]
impl Note {
    pub async fn content_at(&self) -> DateTime<Utc> {
        self.content_at.clone()
    }
    pub async fn text(&self) -> &str {
        &self.text
    }
    pub async fn ride(&self) -> Option<&Ride> {
        self.ride.as_ref()
    }
}
