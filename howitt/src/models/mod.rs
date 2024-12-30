use std::{
    borrow::Cow,
    hash::Hash,
    marker::{ConstParamTy, PhantomData},
    sync::Arc,
};

use anyhow::anyhow;
use futures::FutureExt;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::OnceCell;

use crate::repos::AnyhowRepo;

pub mod cardinal_direction;
pub mod config;
pub mod cuesheet;
pub mod external_ref;
pub mod maybe_pair;
pub mod photo;
pub mod point;
pub mod point_of_interest;
pub mod ride;
pub mod route;
pub mod route_description;
pub mod segment;
pub mod segment_summary;
pub mod slope_end;
pub mod tag;
pub mod terminus;

pub trait Model: Send + Sync + Sized + Clone + 'static {
    type Id: ModelId;
    type IndexItem: IndexItem<Id = Self::Id>;
    type OtherItem: OtherItem<Id = Self::Id>;
    type Filter: Send + Sync + Sized + Clone + 'static;

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
    type Filter: Send + Sync + Sized + Clone + 'static;

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

impl<T, ID, F> Model for T
where
    T: IndexModel<Id = ID, Filter = F>
        + Serialize
        + DeserializeOwned
        + Send
        + Sync
        + Sized
        + Clone
        + 'static,
    ID: ModelId + 'static,
    F: Send + Sync + Sized + Clone + 'static,
{
    type Id = ID;

    type IndexItem = T;

    type OtherItem = EmptyOtherItem<ID>;

    type Filter = F;

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
enum ModelRefInitial<M: Model> {
    Model(M),
    Index(M::IndexItem),
}

pub struct ModelRefInner<M: Model> {
    initial: ModelRefInitial<M>,
    model: OnceCell<Result<M, anyhow::Error>>,
}

#[derive(Clone)]
pub struct ModelRef<M: Model> {
    inner: Arc<ModelRefInner<M>>,
}

impl<M> ModelRef<M>
where
    M: Model + Clone,
{
    fn new(initial: ModelRefInitial<M>) -> ModelRef<M> {
        ModelRef {
            inner: Arc::new(ModelRefInner {
                initial,
                model: OnceCell::new(),
            }),
        }
    }
    pub fn from_model(model: M) -> ModelRef<M> {
        ModelRef::new(ModelRefInitial::Model(model))
    }
    pub fn from_index(index: M::IndexItem) -> ModelRef<M> {
        ModelRef::new(ModelRefInitial::Index(index))
    }
    pub fn id(&self) -> M::Id {
        match &self.inner.initial {
            ModelRefInitial::Model(model) => model.id(),
            ModelRefInitial::Index(index) => index.model_id(),
        }
    }
    pub fn as_index(&self) -> &M::IndexItem {
        match &self.inner.initial {
            ModelRefInitial::Model(model) => model.as_index(),
            ModelRefInitial::Index(index) => index,
        }
    }
    pub async fn as_model<R: AsRef<dyn AnyhowRepo<Model = M>> + Send + Sync>(
        &self,
        repo: R,
    ) -> Result<&M, &anyhow::Error> {
        self.inner
            .model
            .get_or_init(move || match self.inner.initial.clone() {
                ModelRefInitial::Model(model) => futures::future::ready(Ok(model)).boxed(),
                ModelRefInitial::Index(index) => {
                    (async move { repo.as_ref().get(index.model_id()).await }).boxed()
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

pub trait ModelId:
    Send + Sync + std::fmt::Debug + std::fmt::Display + PartialEq + Copy + Clone + Hash + Eq + 'static
{
    fn model_name() -> &'static str;
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum ModelName {
    Photo,
    Checkpoint,
    Ride,
    Route,
    Segment,
}
impl ModelName {
    const fn to_str(self) -> &'static str {
        match self {
            ModelName::Photo => "PHOTO",
            ModelName::Checkpoint => "CHECKPOINT",
            ModelName::Ride => "RIDE",
            ModelName::Route => "ROUTE",
            ModelName::Segment => "SEGMENT",
        }
    }
}

#[derive(derive_more::From, derive_more::Into, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ModelUlid<const NAME: ModelName>(ulid::Ulid);

impl<const NAME: ModelName> ModelUlid<NAME> {
    pub fn new() -> ModelUlid<NAME> {
        ModelUlid::<NAME>(ulid::Ulid::new())
    }
    pub fn as_ulid(&self) -> &ulid::Ulid {
        &self.0
    }
    pub fn from_datetime(datetime: chrono::DateTime<chrono::Utc>) -> ModelUlid<NAME> {
        ModelUlid::<NAME>(ulid::Ulid::from_datetime(datetime.into()))
    }
    pub fn get_or_from_datetime(
        id: Option<ModelUlid<NAME>>,
        datetime: &chrono::DateTime<chrono::Utc>,
    ) -> ModelUlid<NAME> {
        match id {
            Some(id) => id,
            None => ModelUlid::<NAME>::from_datetime(*datetime),
        }
    }
}

impl<const NAME: ModelName> ModelId for ModelUlid<NAME> {
    fn model_name() -> &'static str {
        NAME.to_str()
    }
}

impl<const NAME: ModelName> std::fmt::Debug for ModelUlid<NAME> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", NAME.to_str(), self.0)
    }
}

impl<const NAME: ModelName> std::fmt::Display for ModelUlid<NAME> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", NAME.to_str(), self.0)
    }
}

impl<const NAME: ModelName> Serialize for ModelUlid<NAME> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de, const NAME: ModelName> Deserialize<'de> for ModelUlid<NAME> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let model_id: String = Deserialize::deserialize(deserializer)?;
        let parts = model_id.split('#').collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(serde::de::Error::custom("expected 2 parts"));
        }

        let name = parts[0];
        let id = parts[1];

        if name != NAME.to_str() {
            return Err(serde::de::Error::custom(
                "model name component of ID did not match",
            ));
        }

        std::str::FromStr::from_str(id)
            .map(ModelUlid::<NAME>)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use test_case::test_case;

    use crate::models::route::RouteId;

    fn datetime1() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .unwrap()
            .into()
    }

    fn datetime2() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2023-10-01T00:00:00Z")
            .unwrap()
            .into()
    }

    const ULID_PREFIX1: &str = "ROUTE#01GNNA1J00";
    const ULID_PREFIX2: &str = "ROUTE#01HBM8HS00";

    #[test_case(Some(RouteId::from_datetime(datetime1())), datetime2(), ULID_PREFIX1)]
    #[test_case(Some(RouteId::from_datetime(datetime2())), datetime1(), ULID_PREFIX2)]
    #[test_case(None, datetime2(), ULID_PREFIX2)]
    fn test_get_or_from_datetime(
        existing_id: Option<RouteId>,
        datetime: DateTime<Utc>,
        expected_prefix: &str,
    ) {
        let id = RouteId::get_or_from_datetime(existing_id, &datetime).to_string();
        let (prefix, _) = id.split_at(expected_prefix.len());

        assert_eq!(expected_prefix, prefix);
    }
}
