pub trait Model: Sized {
    type Item: Item;

    fn model_name() -> &'static str;
    fn id(&self) -> String;
    fn into_items(self) -> impl Iterator<Item = Self::Item>;
    fn from_items(items: Vec<Self::Item>) -> Result<Self, anyhow::Error>;
}

pub trait Item {
    fn item_name(&self) -> &'static str;
    fn model_id(&self) -> String;
    fn item_id(&self) -> Option<String>;
}
