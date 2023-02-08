use serde::{de::DeserializeOwned, Serialize};

pub trait Model: Send + Sync + Sized {
    type Item: Item;

    fn model_name() -> &'static str;
    fn id(&self) -> String;
    fn into_items(self) -> impl Iterator<Item = Self::Item>;
    fn from_items(items: Vec<Self::Item>) -> Result<Self, anyhow::Error>;
}

pub trait Item: Send + Sync + Serialize + DeserializeOwned {
    fn item_name(&self) -> &'static str;
    fn model_id(&self) -> String;
    fn item_id(&self) -> Option<String>;
}
