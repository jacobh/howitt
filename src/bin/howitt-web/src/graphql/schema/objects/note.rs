use async_graphql::Object;
use chrono::{DateTime, Utc};

pub struct Note {
    pub content_at: DateTime<Utc>,
    pub text: String,
}

#[Object]
impl Note {
    pub async fn content_at(&self) -> DateTime<Utc> {
        self.content_at.clone()
    }
    pub async fn text(&self) -> &str {
        &self.text
    }
}
