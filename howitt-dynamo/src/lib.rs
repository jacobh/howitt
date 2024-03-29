#![feature(async_closure)]
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::anyhow;
use aws_sdk_dynamodb as dynamodb;
use derive_more::Constructor;
use dynamodb::{
    error::SdkError,
    operation::{
        delete_item::{DeleteItemError, DeleteItemOutput},
        get_item::GetItemError,
        put_item::{PutItemError, PutItemOutput},
        query::QueryError,
        scan::ScanError,
    },
    types::AttributeValue,
};
use futures::prelude::*;
use howitt::ext::futures::FuturesIteratorExt;
use howitt::models::{
    config::Config, point_of_interest::PointOfInterest, ride::RideModel, route::RouteModel, Model,
};
use howitt::models::{ItemCow, ModelId};
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
    pub pk: String,
    pub sk: String,
    pub gsi1pk: Option<String>,
    pub gsi1sk: Option<String>,
}
impl Keys {
    pub fn new_pk_sk(pk: String, sk: String) -> Keys {
        Keys {
            pk,
            sk,
            gsi1pk: None,
            gsi1sk: None,
        }
    }
    pub fn from_model_id<ID>(id: ID) -> Keys
    where
        ID: ModelId,
    {
        Keys::new_pk_sk(id.to_string(), id.to_string())
    }
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

    pub async fn get(
        &self,
        keys: Keys,
    ) -> Result<Option<HashMap<String, AttributeValue>>, SdkError<GetItemError>> {
        let _permit = self.acquire_semaphore_permit().await;

        let output = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(keys.pk))
            .key("sk", AttributeValue::S(keys.sk))
            .send()
            .await?;

        Ok(output.item)
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
    parts.into_iter().flatten().join("#")
}

#[derive(Debug, thiserror::Error)]
#[error("Dynamo Repo Error")]
pub enum DynamoRepoError {
    AwsQueryError(#[from] SdkError<QueryError>),
    AwsGetItemError(#[from] SdkError<GetItemError>),
    AwsPutItemError(#[from] SdkError<PutItemError>),
    SerdeDynamo(#[from] serde_dynamo::Error),
    ModelFromItems(anyhow::Error),
    NotFound,
}

#[derive(Debug, Clone)]
pub struct DynamoModelRepo<M: Model> {
    client: SingleTableClient,
    model: PhantomData<M>,
}

impl<M> DynamoModelRepo<M>
where
    M: Model,
{
    pub fn new(client: SingleTableClient) -> DynamoModelRepo<M> {
        DynamoModelRepo {
            client,
            model: PhantomData,
        }
    }

    fn client(&self) -> &SingleTableClient {
        &self.client
    }

    fn keys(item: &ItemCow<'_, M>) -> Keys {
        let model_name = M::model_name().to_string();
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
}

#[async_trait::async_trait]
impl<M> Repo for DynamoModelRepo<M>
where
    M: Model,
{
    type Model = M;
    type Error = DynamoRepoError;

    async fn all_indexes(&self) -> Result<Vec<M::IndexItem>, DynamoRepoError> {
        let items = self
            .client()
            .query_pk(["", M::model_name()].join("#"), Index::Gsi1)
            .await?;

        Ok(items
            .into_iter()
            .map(serde_dynamo::from_item)
            .collect::<Result<Vec<_>, _>>()?)
    }
    async fn get(&self, id: M::Id) -> Result<M, DynamoRepoError> {
        let items = self
            .client()
            .query_pk(id.to_string(), Index::Default)
            .await?;

        if items.is_empty() {
            return Err(DynamoRepoError::NotFound);
        }

        let items = items
            .into_iter()
            .map(serde_dynamo::from_item::<_, ItemCow<'static, M>>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(M::from_items(items).map_err(DynamoRepoError::ModelFromItems)?)
    }
    async fn get_index(&self, id: M::Id) -> Result<M::IndexItem, DynamoRepoError> {
        let item = self.client().get(Keys::from_model_id(id)).await?;

        match item {
            Some(item) => Ok(serde_dynamo::from_item(item)?),
            None => Err(DynamoRepoError::NotFound),
        }
    }
    async fn put(&self, model: M) -> Result<(), DynamoRepoError> {
        let items = model.into_items().into_iter().collect::<Vec<_>>();

        items
            .into_iter()
            .map(|item| (item, self.client().clone()))
            .map(async move |(item, client)| -> Result<_, DynamoRepoError> {
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
}

pub type PointOfInterestRepo = DynamoModelRepo<PointOfInterest>;
pub type RideRepo = DynamoModelRepo<RideModel>;
pub type RouteModelRepo = DynamoModelRepo<RouteModel>;
pub type RideModelRepo = DynamoModelRepo<RideModel>;
pub type ConfigRepo = DynamoModelRepo<Config>;
