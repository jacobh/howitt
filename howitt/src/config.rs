use serde::{Deserialize, Serialize};

use crate::{
    model::{Item, Model, ModelId},
    route::RouteId,
};

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

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub starred_route_ids: Vec<RouteId>,
}
impl Model for Config {
    type Id = ConfigId;
    type Item = Config;

    fn id(&self) -> ConfigId {
        ConfigId
    }

    fn into_items(self) -> impl IntoIterator<Item = Self::Item> {
        [self]
    }

    fn from_items(items: Vec<Self::Item>) -> Result<Self, anyhow::Error> {
        if items.len() != 1 {
            return Err(anyhow::anyhow!("Expected exactly 1 config item"));
        }
        return items
            .into_iter()
            .next()
            .ok_or(anyhow::anyhow!("Expected exactly 1 config item"));
    }
}

impl Item for Config {
    type Id = ConfigId;

    fn model_id(&self) -> ConfigId {
        ConfigId
    }

    fn item_name(&self) -> Option<String> {
        None
    }

    fn item_id(&self) -> Option<String> {
        None
    }
}
