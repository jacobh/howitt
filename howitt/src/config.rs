use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub starred_route_ids: Vec<ulid::Ulid>,
}
