#![feature(async_closure)]
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use aws_sdk_dynamodb as dynamodb;
use derive_more::Constructor;
use dynamodb::error::{PutItemError, QueryError, ScanError};
use dynamodb::output::PutItemOutput;
use dynamodb::{
    error::GetItemError, model::AttributeValue, output::GetItemOutput, types::SdkError,
};
use futures::{prelude::*, stream::FuturesOrdered};
use howitt::checkpoint::Checkpoint;
use howitt::model::{Item, Model};
use howitt::repo::Repo;
use howitt::route::{Route, RouteModel};
use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::sync::{Semaphore, SemaphorePermit};

pub enum Index {
    Default,
    Gsi1
}
impl Index {
    fn to_index_name(&self) -> Option<String> {
        match self {
            Index::Default => None,
            Index::Gsi1 => Some("gsi1".to_string()),
        }
    }
    fn to_pk_name(&self) -> String {
        match self {
            Index::Default => "pk".to_string(),
            Index::Gsi1 => "gsi1pk".to_string(),
        }
    }
}

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct Keys {
    pk: String,
    sk: String,
    gsi1pk: String,
    gsi1sk: String,
}
impl Keys {
    fn from_item(item: &HashMap<String, AttributeValue>) -> Result<Keys, anyhow::Error> {
        Ok(Keys {
            pk: item
                .get("pk")
                .ok_or(anyhow!("pk missing"))?
                .as_s()
                .map_err(|_| anyhow!("pk not string"))?
                .to_owned(),
            sk: item
                .get("sk")
                .ok_or(anyhow!("sk missing"))?
                .as_s()
                .map_err(|_| anyhow!("sk not string"))?
                .to_owned(),
            gsi1pk: item
                .get("gsi1pk")
                .ok_or(anyhow!("gsi1pk missing"))?
                .as_s()
                .map_err(|_| anyhow!("gsi1pk not string"))?
                .to_owned(),
            gsi1sk: item
                .get("gsi1sk")
                .ok_or(anyhow!("gsi1sk missing"))?
                .as_s()
                .map_err(|_| anyhow!("gsi1sk not string"))?
                .to_owned(),
        })
    }
}

#[derive(Debug, Constructor, Clone)]
pub struct SingleTableClient {
    client: dynamodb::Client,
    semaphore: Arc<Semaphore>,
    table_name: String,
}
impl SingleTableClient {
    pub async fn new_from_env() -> SingleTableClient {
        let config = aws_config::load_from_env().await;
        SingleTableClient {
            client: dynamodb::Client::new(&config),
            semaphore: Arc::new(Semaphore::new(20)),
            table_name: std::env::var("HOWITT_TABLE_NAME").unwrap_or("howitt".to_string()),
        }
    }

    async fn acquire_semaphore_permit(&self) -> SemaphorePermit {
        self.semaphore.acquire().await.unwrap()
    }

    pub async fn get(&self, keys: Keys) -> Result<GetItemOutput, SdkError<GetItemError>> {
        let _permit = self.acquire_semaphore_permit().await;

        self.client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(keys.pk))
            .key("sk", AttributeValue::S(keys.sk))
            .send()
            .await
    }

    pub async fn query_pk(
        &self,
        pk: String,
        index: Index
    ) -> Result<Vec<HashMap<String, AttributeValue>>, SdkError<QueryError>> {
        let _permit = self.acquire_semaphore_permit().await;

        let outputs = self
            .client
            .query()
            .table_name(&self.table_name)
            .set_index_name(index.to_index_name())
            .key_condition_expression("#pk = :pk")
            .expression_attribute_names("#pk", index.to_pk_name())
            .expression_attribute_values(":pk", AttributeValue::S(pk))
            .into_paginator()
            .send()
            .collect::<Vec<_>>()
            .await;

        Ok(outputs
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter_map(|output| output.items)
            .flatten()
            .collect())
    }

    pub async fn put(
        &self,
        keys: Keys,
        mut item: HashMap<String, AttributeValue>,
    ) -> Result<PutItemOutput, SdkError<PutItemError>> {
        let _permit = self.acquire_semaphore_permit().await;

        let keys: HashMap<String, AttributeValue> = serde_dynamo::to_item(keys).unwrap();

        item.extend(keys);

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
    }

    pub async fn put_batch(
        &self,
        items: Vec<(Keys, HashMap<String, AttributeValue>)>,
    ) -> Vec<Result<PutItemOutput, SdkError<PutItemError>>> {
        items
            .into_iter()
            .map(|(keys, item)| (keys, item, self.clone()))
            .map(async move |(keys, item, client)| client.put(keys, item).await)
            .collect::<FuturesOrdered<_>>()
            .collect()
            .await
    }

    pub async fn scan(&self) -> Result<Vec<HashMap<String, AttributeValue>>, SdkError<ScanError>> {
        let _permit = self.acquire_semaphore_permit().await;

        let outputs = self
            .client
            .scan()
            .table_name(&self.table_name)
            .into_paginator()
            .send()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(outputs
            .into_iter()
            .filter_map(|output| output.items)
            .flatten()
            .collect())
    }
}

#[async_trait::async_trait]
pub trait DynamoRepo<T: Send + Sync + Serialize + DeserializeOwned> {
    fn client(&self) -> &SingleTableClient;

    fn is_item(keys: &Keys) -> bool;

    fn keys(item: &T) -> Keys;

    async fn get(&self, id: String) -> Result<T, anyhow::Error> {
        let item = self.client().get(Keys::new(id.clone(), id.clone(), id.clone(), id.clone())).await?;
        let item = item.item().unwrap().clone();
        Ok(serde_dynamo::from_item(item)?)
    }

    async fn put(&self, item: T) -> Result<(), anyhow::Error>
    where
        T: 'async_trait,
    {
        self.client()
            .put(Self::keys(&item), serde_dynamo::to_item(item)?)
            .await?;

        Ok(())
    }

    async fn put_batch(&self, items: Vec<T>) -> Result<(), anyhow::Error>
    where
        T: 'async_trait,
    {
        let items = items
            .into_iter()
            .map(|item| -> Result<_, anyhow::Error> {
                Ok((Self::keys(&item), serde_dynamo::to_item(item)?))
            })
            .collect::<Result<Vec<_>, _>>()?;

        self.client()
            .put_batch(items)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    async fn all(&self) -> Result<Vec<T>, anyhow::Error> {
        let items = self.client().query_pk("#CHECKPOINT".to_string(), Index::Gsi1).await?;

        dbg!(&items);
        Ok(items
            .into_iter()
            .filter(|item| match Keys::from_item(item) {
                Ok(keys) => Self::is_item(&keys),
                Err(_) => false,
            })
            .map(serde_dynamo::from_item)
            .collect::<Result<_, _>>()?)
    }
}

#[derive(Debug, Constructor, Clone)]
pub struct CheckpointRepo {
    client: SingleTableClient,
}
impl DynamoRepo<Checkpoint> for CheckpointRepo {
    fn client(&self) -> &SingleTableClient {
        &self.client
    }

    fn is_item(keys: &Keys) -> bool {
        keys.pk.starts_with("CHECKPOINT#")
    }

    fn keys(item: &Checkpoint) -> Keys {
        Keys::new(
            format!("CHECKPOINT#{}", item.id.hyphenated()),
            format!("CHECKPOINT#{}", item.id.hyphenated()),
            "#CHECKPOINT".to_string(),
            format!("CHECKPOINT#{}", item.id.hyphenated()),
        )
    }
}

#[async_trait::async_trait]
impl Repo<Checkpoint, anyhow::Error> for CheckpointRepo {
    async fn all(&self) -> Result<Vec<Checkpoint>, anyhow::Error> {
        DynamoRepo::all(self).await
    }
}

#[derive(Debug, Constructor, Clone)]
pub struct RouteRepo {
    client: SingleTableClient,
}
impl DynamoRepo<Route> for RouteRepo {
    fn client(&self) -> &SingleTableClient {
        &self.client
    }

    fn is_item(keys: &Keys) -> bool {
        keys.pk.starts_with("ROUTE#")
    }

    fn keys(item: &Route) -> Keys {
        Keys::new(format!("ROUTE#{}", item.id), format!("ROUTE#{}", item.id), "#ROUTE".to_string(), format!("ROUTE#{}", item.id))
    }
}

#[async_trait::async_trait]
impl Repo<Route, anyhow::Error> for RouteRepo {
    async fn all(&self) -> Result<Vec<Route>, anyhow::Error> {
        DynamoRepo::all(self).await
    }
}

// ..

#[async_trait::async_trait]
pub trait DynamoModelRepo {
    type Model: Model;

    fn client(&self) -> &SingleTableClient;

    fn pk(model_id: impl Into<String>) -> String {
        vec![Self::Model::model_name().to_string(), model_id.into()].join("#")
    }

    fn keys(item: &<Self::Model as Model>::Item) -> Keys {
        let model_name = Self::Model::model_name().to_string();
        let model_id = item.model_id();
        let item_name = item.item_name().to_string();
        let item_id = item.item_id();

        Keys {
            pk: vec![&*model_name, &*model_id].join("#"),
            sk: vec![Some(&*model_name), Some(&*model_id), Some(&*item_name), item_id.as_deref()]
                .into_iter()
                .filter_map(|x| x)
                .collect::<Vec<_>>()
                .join("#"),
            gsi1pk: vec!["", &*model_name, &*item_name].join("#"),
            gsi1sk: vec![Some(&*model_name), Some(&*model_id), Some(&*item_name), item_id.as_deref()]
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<_>>()
            .join("#"),
        }
    }

    async fn get_model(&self, model_id: String) -> Result<Self::Model, anyhow::Error> {
        let items = self.client().query_pk(Self::pk(model_id), Index::Default).await?;

        let items = items
            .into_iter()
            .map(serde_dynamo::from_item::<_, <Self::Model as Model>::Item>)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::Model::from_items(items)?)
    }

    async fn put(&self, model: Self::Model) -> Result<(), anyhow::Error> {
        let items = model.into_items().collect::<Vec<_>>();

        items
            .into_iter()
            .map(|item| (item, self.client().clone()))
            .map(async move |(item, client)| -> Result<_, anyhow::Error> {
                Ok(client
                    .put(Self::keys(&item), serde_dynamo::to_item(item)?)
                    .await?)
            })
            .collect::<FuturesOrdered<_>>()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    async fn put_batch(&self, models: Vec<Self::Model>) -> Result<(), anyhow::Error> {
        models
            .into_iter()
            .map(|model| (model, self.clone()))
            .map(async move |(model, repo)| repo.put(model).await)
            .collect::<FuturesOrdered<_>>()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    async fn all(&self) -> Result<Vec<Self::Model>, anyhow::Error> {
        let items = self.client().scan().await?;

        let items = items
            .into_iter()
            .map(serde_dynamo::from_item::<_, <Self::Model as Model>::Item>)
            .filter_map(Result::ok);

        let groups = items.group_by(|item| item.model_id());

        Ok(groups
            .into_iter()
            .map(|(_, items)| Self::Model::from_items(items.collect_vec()))
            .collect::<Result<_, _>>()?)
    }
}

#[derive(Debug, Constructor, Clone)]
pub struct RouteModelRepo {
    client: SingleTableClient,
}
impl DynamoModelRepo for RouteModelRepo {
    type Model = RouteModel;

    fn client(&self) -> &SingleTableClient {
        &self.client
    }
}
#[async_trait::async_trait]
impl Repo<RouteModel, anyhow::Error> for RouteModelRepo {
    async fn all(&self) -> Result<Vec<RouteModel>, anyhow::Error> {
        DynamoModelRepo::all(self).await
    }
}
