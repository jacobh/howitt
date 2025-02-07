use derive_more::derive::From;
use serde::{Deserialize, Serialize};

pub mod media;
pub mod rwgps;

#[derive(Debug, Deserialize, Serialize, From, Clone)]
pub enum Job {
    Media(media::MediaJob),
    Rwgps(rwgps::RwgpsJob),
}
