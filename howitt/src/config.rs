use serde::{Deserialize, Serialize};

use crate::model::{Item, Model};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub starred_route_ids: Vec<ulid::Ulid>,
}
impl Model for Config {
    type Item = Config;

    fn model_name() -> &'static str {
        return "CONFIG";
    }

    fn id(&self) -> String {
        return String::from("SINGLETON");
    }

    fn into_items(self) -> impl IntoIterator<Item = Self::Item> {
        return [self];
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
    fn model_id(&self) -> String {
        return String::from("SINGLETON");
    }

    fn item_name(&self) -> Option<String> {
        return None;
    }

    fn item_id(&self) -> Option<String> {
        return None;
    }
}
