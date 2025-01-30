use derive_more::derive::From;
use serde::{Deserialize, Serialize};

pub mod media;

#[derive(Debug, Deserialize, Serialize, From)]
pub enum Job {
    Media(media::MediaJob),
}
