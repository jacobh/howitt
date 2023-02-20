#![feature(async_closure)]
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use aws_sdk_dynamodb as dynamodb;
use derive_more::Constructor;
use dynamodb::error::{DeleteItemError, PutItemError, QueryError, ScanError};
use dynamodb::output::{DeleteItemOutput, PutItemOutput};
use dynamodb::{
    error::GetItemError, model::AttributeValue, output::GetItemOutput, types::SdkError,
};
use futures::prelude::*;
use howitt::ext::futures::FuturesIteratorExt;
use howitt::models::{
    checkpoint::Checkpoint, config::Config, ride::RideModel, route::RouteModel, Item, Model,
};
use howitt::repos::Repo;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tokio::sync::{Semaphore, SemaphorePermit};

pub enum Index {
    Default,
    Gsi1,
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
    gsi1pk: Option<String>,
    gsi1sk: Option<String>,
}
impl Keys {
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Result<Keys, anyhow::Error> {
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
                .map(|value| value.as_s())
                .transpose()
                .map_err(|_| anyhow!("gsi1pk not string"))?
                .map(ToOwned::to_owned),
            gsi1sk: item
                .get("gsi1sk")
                .map(|value| value.as_s())
                .transpose()
                .map_err(|_| anyhow!("gsi1sk not string"))?
                .map(ToOwned::to_owned),
        })
    }
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        serde_dynamo::to_item(self).unwrap()
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
            semaphore: Arc::new(Semaphore::new(10)),
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
        index: Index,
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
            .collect_futures_ordered()
            .await
    }

    pub async fn delete(&self, keys: Keys) -> Result<DeleteItemOutput, SdkError<DeleteItemError>> {
        let _permit = self.acquire_semaphore_permit().await;

        self.client
            .delete_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(keys.pk))
            .key("sk", AttributeValue::S(keys.sk))
            .send()
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

    pub async fn scan_keys(
        &self,
    ) -> Result<Vec<HashMap<String, AttributeValue>>, SdkError<ScanError>> {
        let _permit = self.acquire_semaphore_permit().await;

        let outputs = self
            .client
            .scan()
            .table_name(&self.table_name)
            .set_attributes_to_get(Some(vec![
                String::from("pk"),
                String::from("sk"),
                String::from("gsi1pk"),
                String::from("gsi1sk"),
            ]))
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

fn format_key<'a>(parts: impl IntoIterator<Item = Option<&'a str>>) -> String {
    parts.into_iter().filter_map(|x| x).join("#")
}

#[async_trait::async_trait]
pub trait DynamoModelRepo: Send + Sync {
    type Model: Model;

    fn client(&self) -> &SingleTableClient;

    fn keys(item: &<Self::Model as Model>::Item) -> Keys {
        let model_name = Self::Model::model_name().to_string();
        let model_id = item.model_id().to_string();
        let item_name = item.item_name();
        let item_id = item.item_id();

        Keys {
            pk: model_id.to_string(),
            sk: format_key([
                Some(model_id.as_str()),
                item_name.as_deref(),
                item_id.as_deref(),
            ]),
            gsi1pk: Some(format_key([
                Some(""),
                Some(&*model_name),
                item_name.as_deref(),
            ])),
            gsi1sk: Some(format_key([
                Some(&*model_id),
                item_name.as_deref(),
                item_id.as_deref(),
            ])),
        }
    }

    async fn get_model(
        &self,
        model_id: <<Self as DynamoModelRepo>::Model as Model>::Id,
    ) -> Result<Option<Self::Model>, anyhow::Error> {
        let items = self
            .client()
            .query_pk(model_id.to_string(), Index::Default)
            .await?;

        if items.len() == 0 {
            return Ok(None);
        }

        let items = items
            .into_iter()
            .map(serde_dynamo::from_item::<_, <Self::Model as Model>::Item>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Some(Self::Model::from_items(items)?))
    }

    async fn get_batch(
        &self,
        model_ids: Vec<<<Self as DynamoModelRepo>::Model as Model>::Id>,
    ) -> Result<Vec<Self::Model>, anyhow::Error> {
        let results = model_ids
            .into_iter()
            .map(|id| (id, self.clone()))
            .map(async move |(id, repo)| repo.get_model(id).await)
            .collect_futures_ordered()
            .await;

        let items = results
            .into_iter()
            .map(Result::transpose)
            .flatten()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
        // results.
    }

    async fn put(&self, model: Self::Model) -> Result<(), anyhow::Error> {
        let items = model.into_items().into_iter().collect::<Vec<_>>();

        items
            .into_iter()
            .map(|item| (item, self.client().clone()))
            .map(async move |(item, client)| -> Result<_, anyhow::Error> {
                Ok(client
                    .put(Self::keys(&item), serde_dynamo::to_item(item)?)
                    .await?)
            })
            .collect_futures_ordered()
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
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    async fn all(&self) -> Result<Vec<Self::Model>, anyhow::Error> {
        let items = self
            .client()
            .query_pk(["", Self::Model::model_name()].join("#"), Index::Gsi1)
            .await?;

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

macro_rules! impl_repo {
    ($repo_type:ident, $model_type:ident) => {
        #[derive(Debug, Constructor, Clone)]
        pub struct $repo_type {
            client: SingleTableClient,
        }
        impl DynamoModelRepo for $repo_type {
            type Model = $model_type;

            fn client(&self) -> &SingleTableClient {
                &self.client
            }
        }

        #[async_trait::async_trait]
        impl Repo<$model_type> for $repo_type {
            type Error = anyhow::Error;

            async fn all(&self) -> Result<Vec<$model_type>, anyhow::Error> {
                DynamoModelRepo::all(self).await
            }
            async fn get(
                &self,
                id: <$model_type as Model>::Id,
            ) -> Result<Option<$model_type>, anyhow::Error> {
                DynamoModelRepo::get_model(self, id).await
            }
            async fn get_batch(
                &self,
                ids: Vec<<$model_type as Model>::Id>,
            ) -> Result<Vec<$model_type>, anyhow::Error> {
                DynamoModelRepo::get_batch(self, ids).await
            }
        }
    };
}

impl_repo!(CheckpointRepo, Checkpoint);
impl_repo!(RideRepo, RideModel);
impl_repo!(RouteModelRepo, RouteModel);
impl_repo!(ConfigRepo, Config);
