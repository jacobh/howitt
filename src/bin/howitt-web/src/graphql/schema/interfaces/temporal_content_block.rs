use async_graphql::Interface;

use crate::graphql::schema::{media::Media, note::Note, ride::Ride};

use chrono::{DateTime, Utc};

#[derive(Interface)]
#[graphql(field(
    name = "content_at",
    ty = "DateTime<Utc>",
    desc = "Timestamp associated with this content"
))]
pub enum TemporalContentBlock {
    Ride(Ride),
    Media(Media),
    Note(Note),
}
