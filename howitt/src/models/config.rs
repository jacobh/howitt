use serde::{Deserialize, Serialize};

use crate::models::{IndexModel, ModelId, route::RouteId};

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub starred_route_ids: Vec<RouteId>,
    #[serde(default = "Vec::new")]
    pub api_keys: Vec<String>,
}

impl IndexModel for Config {
    type Id = ConfigId;
    type Filter = ();

    fn id(&self) -> ConfigId {
        ConfigId
    }
}
