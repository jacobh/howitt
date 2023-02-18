use serde::{de::DeserializeOwned, Serialize};

pub trait Model: Send + Sync + Sized {
    type Item: Item;

    fn model_name() -> &'static str;
    fn id(&self) -> String;
    fn into_items(self) -> impl IntoIterator<Item = Self::Item>;
    fn from_items(items: Vec<Self::Item>) -> Result<Self, anyhow::Error>;
}

pub trait Item: Send + Sync + Serialize + DeserializeOwned {
    fn model_id(&self) -> String;
    fn item_name(&self) -> Option<String>;
    fn item_id(&self) -> Option<String>;
}

#[macro_export]
macro_rules! model_id {
    ($type_name:ident, $model_name:expr) => {
        #[derive(Debug, derive_more::From, derive_more::Into)]
        pub struct $type_name(ulid::Ulid);

        impl $type_name {
            fn new() -> $type_name {
                $type_name(ulid::Ulid::new())
            }
        }

        impl Serialize for $type_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                [String::from($model_name), self.0.to_string()]
                    .join("#")
                    .serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $type_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let model_id: String = Deserialize::deserialize(deserializer)?;
                let parts = model_id.split("#").collect::<Vec<_>>();

                if (parts.len() != 2) {
                    return Err(serde::de::Error::custom("expected 2 parts"));
                }

                let name = parts[0];
                let id = parts[1];

                if (name != $model_name) {
                    return Err(serde::de::Error::custom(
                        "model name component of ID did not match",
                    ));
                }

                std::str::FromStr::from_str(id)
                    .map($type_name)
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}