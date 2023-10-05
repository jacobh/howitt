use std::{borrow::Cow, marker::PhantomData};

use anyhow::anyhow;
use futures::FutureExt;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::OnceCell;

use crate::repos::AnyhowRepo;

pub mod config;
pub mod cuesheet;
pub mod external_ref;
pub mod maybe_pair;
pub mod point;
pub mod point_of_interest;
pub mod ride;
pub mod route;
pub mod route_description;
pub mod segment;
pub mod segment_summary;

pub trait Model: Send + Sync + Sized + 'static {
    type Id: ModelId;
    type IndexItem: IndexItem<Id = Self::Id>;
    type OtherItem: OtherItem<Id = Self::Id>;

    fn model_name() -> &'static str {
        Self::Id::model_name()
    }
    fn id(&self) -> Self::Id;

    fn as_index(&self) -> &Self::IndexItem;

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

    fn as_index(&self) -> &Self::IndexItem {
        self
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

#[derive(Clone)]
enum ModelRefInner<M: Model> {
    Model(M),
    Index(M::IndexItem),
}

pub struct ModelRef<M: Model> {
    initial: ModelRefInner<M>,
    model: OnceCell<Result<M, anyhow::Error>>,
}

impl<M> ModelRef<M>
where
    M: Model + Clone,
{
    fn new(initial: ModelRefInner<M>) -> ModelRef<M> {
        ModelRef {
            initial,
            model: OnceCell::new(),
        }
    }
    pub fn from_model(model: M) -> ModelRef<M> {
        ModelRef::new(ModelRefInner::Model(model))
    }
    pub fn from_index(index: M::IndexItem) -> ModelRef<M> {
        ModelRef::new(ModelRefInner::Index(index))
    }
    pub fn id(&self) -> M::Id {
        match &self.initial {
            ModelRefInner::Model(model) => model.id(),
            ModelRefInner::Index(index) => index.model_id(),
        }
    }
    pub fn as_index(&self) -> &M::IndexItem {
        match &self.initial {
            ModelRefInner::Model(model) => model.as_index(),
            ModelRefInner::Index(index) => index,
        }
    }
    pub async fn as_model<R: AsRef<dyn AnyhowRepo<Model = M>> + Send + Sync>(
        &self,
        repo: R,
    ) -> Result<&M, &anyhow::Error> {
        self.model
            .get_or_init(move || match self.initial.clone() {
                ModelRefInner::Model(model) => futures::future::ready(Ok(model)).boxed(),
                ModelRefInner::Index(index) => {
                    ((async move || Ok(repo.as_ref().get(index.model_id()).await?))()).boxed()
                }
            })
            .await
            .as_ref()
    }
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

type GroupedItemCows<'a, M> = (
    Cow<'a, <M as Model>::IndexItem>,
    Vec<Cow<'a, <M as Model>::OtherItem>>,
);

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
    ) -> Result<GroupedItemCows<'a, M>, anyhow::Error> {
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

pub trait ModelId: Send + Sync + std::fmt::Display + PartialEq + Copy + Clone + 'static {
    fn model_name() -> &'static str;
}

#[macro_export]
macro_rules! model_id {
    ($type_name:ident, $model_name:expr) => {
        #[derive(Debug, Default, derive_more::From, derive_more::Into, PartialEq, Clone, Copy)]
        pub struct $type_name(ulid::Ulid);

        impl $type_name {
            pub fn new() -> $type_name {
                $type_name(ulid::Ulid::new())
            }
        }

        impl $crate::models::ModelId for $type_name {
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
