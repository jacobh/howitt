use std::{borrow::Cow, marker::PhantomData};

use anyhow::anyhow;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod checkpoint;
pub mod config;
pub mod external_ref;
pub mod point;
pub mod ride;
pub mod route;
pub mod segment;

pub trait Model: Send + Sync + Sized {
    type Id: ModelId;
    type IndexItem: IndexItem<Id = Self::Id>;
    type OtherItem: OtherItem<Id = Self::Id>;

    fn model_name() -> &'static str {
        Self::Id::model_name()
    }
    fn id(&self) -> Self::Id;

    fn into_parts(self) -> (Self::IndexItem, Vec<Self::OtherItem>);

    fn from_parts(
        index: Self::IndexItem,
        other: Vec<Self::OtherItem>,
    ) -> Result<Self, anyhow::Error>;

    fn into_items(self) -> Vec<ItemCow<'static, Self>> {
        let (index, other) = self.into_parts();

        [ItemCow::from_index(index)]
            .into_iter()
            .chain(other.into_iter().map(ItemCow::from_other))
            .collect()
    }

    fn from_items(items: Vec<ItemCow<'static, Self>>) -> Result<Self, anyhow::Error> {
        let (index, others) = ItemCow::group_items(items)?;

        Self::from_parts(
            index.into_owned(),
            others.into_iter().map(Cow::into_owned).collect(),
        )
    }
}

pub trait IndexModel {
    type Id: ModelId;

    fn id(&self) -> Self::Id;
}

impl<T, ID> IndexItem for T
where
    T: IndexModel<Id = ID> + 'static + Serialize + DeserializeOwned + Send + Sync + Sized + Clone,
    ID: ModelId,
{
    type Id = ID;

    fn model_id(&self) -> Self::Id {
        self.id()
    }
}

impl<T, ID> Model for T
where
    T: IndexModel<Id = ID> + Serialize + DeserializeOwned + Send + Sync + Sized + Clone + 'static,
    ID: ModelId + 'static,
{
    type Id = ID;

    type IndexItem = T;

    type OtherItem = EmptyOtherItem<ID>;

    fn id(&self) -> Self::Id {
        self.id()
    }

    fn into_parts(self) -> (Self::IndexItem, Vec<Self::OtherItem>) {
        (self, vec![])
    }

    fn from_parts(
        index: Self::IndexItem,
        _other: Vec<Self::OtherItem>,
    ) -> Result<Self, anyhow::Error> {
        Ok(index)
    }
}

pub trait IndexItem: 'static + Send + Sync + Clone + Serialize + DeserializeOwned {
    type Id: ModelId;

    fn model_id(&self) -> Self::Id;
}

pub trait OtherItem: 'static + Send + Sync + Clone + Serialize + DeserializeOwned {
    type Id: ModelId;

    fn model_id(&self) -> Self::Id;
    fn item_name(&self) -> String;
    fn item_id(&self) -> String;
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemCow<'a, M>
where
    M: Model,
{
    Index(Cow<'a, M::IndexItem>),
    Other(Cow<'a, M::OtherItem>),
}

impl<'a, M> ItemCow<'a, M>
where
    M: Model,
{
    fn from_index(index: M::IndexItem) -> ItemCow<'static, M> {
        ItemCow::Index(Cow::Owned(index))
    }

    fn from_other(other: M::OtherItem) -> ItemCow<'static, M> {
        ItemCow::Other(Cow::Owned(other))
    }

    // fn as_index(&self) -> Option<&M::IndexItem> {
    //     match self {
    //         ItemCow::Index(item) => Some(item.as_ref()),
    //         ItemCow::Other(_) => None,
    //     }
    // }

    fn group_items(
        items: impl IntoIterator<Item = Self>,
    ) -> Result<(Cow<'a, M::IndexItem>, Vec<Cow<'a, M::OtherItem>>), anyhow::Error> {
        let (indexes, others) =
            items
                .into_iter()
                .fold((vec![], vec![]), |(mut indexes, mut others), item| {
                    match item {
                        ItemCow::Index(index) => indexes.push(index),
                        ItemCow::Other(other) => others.push(other),
                    };
                    (indexes, others)
                });

        let index = indexes
            .into_iter()
            .collect_tuple::<(Cow<'_, M::IndexItem>,)>()
            .map(|(idx,)| idx)
            .ok_or(anyhow!("could not find single indexitem"))?;

        Ok((index, others))
    }

    pub fn model_id(&self) -> M::Id {
        match self {
            ItemCow::Index(item) => item.model_id(),
            ItemCow::Other(item) => item.model_id(),
        }
    }
    pub fn item_name(&self) -> Option<String> {
        match self {
            ItemCow::Index(_) => None,
            ItemCow::Other(item) => Some(item.item_name()),
        }
    }
    pub fn item_id(&self) -> Option<String> {
        match self {
            ItemCow::Index(_) => None,
            ItemCow::Other(item) => Some(item.item_id()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmptyOtherItem<ID>(PhantomData<ID>);

impl<ID> OtherItem for EmptyOtherItem<ID>
where
    ID: ModelId + 'static,
{
    type Id = ID;

    fn model_id(&self) -> Self::Id {
        todo!()
    }

    fn item_name(&self) -> String {
        todo!()
    }

    fn item_id(&self) -> String {
        todo!()
    }
}

pub trait ModelId: Send + Sync + std::fmt::Display + PartialEq + Copy + Clone {
    fn model_name() -> &'static str;
}

#[macro_export]
macro_rules! model_id {
    ($type_name:ident, $model_name:expr) => {
        #[derive(Debug, derive_more::From, derive_more::Into, PartialEq, Clone, Copy)]
        pub struct $type_name(ulid::Ulid);

        impl $type_name {
            pub fn new() -> $type_name {
                $type_name(ulid::Ulid::new())
            }
        }

        impl crate::models::ModelId for $type_name {
            fn model_name() -> &'static str {
                $model_name
            }
        }

        impl std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}#{}", $model_name, self.0)
            }
        }

        impl Serialize for $type_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_string().serialize(serializer)
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
