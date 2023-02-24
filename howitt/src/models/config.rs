use serde::{Deserialize, Serialize};

use crate::models::{route::RouteId, IndexModel, ModelId};

#[derive(PartialEq, Copy, Clone)]
pub struct ConfigId;

impl std::fmt::Display for ConfigId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CONFIG#SINGLETON")
    }
}

impl ModelId for ConfigId {
    fn model_name() -> &'static str {
        "CONFIG"
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub starred_route_ids: Vec<RouteId>,
}

impl IndexModel for Config {
    type Id = ConfigId;

    fn id(&self) -> ConfigId {
        ConfigId
    }
}
