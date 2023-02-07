use std::collections::HashMap;

use anyhow::anyhow;
use aws_sdk_dynamodb as dynamodb;
use derive_more::Constructor;
use dynamodb::error::{PutItemError, ScanError};
use dynamodb::output::{PutItemOutput, ScanOutput};
use dynamodb::{
    error::GetItemError, model::AttributeValue, output::GetItemOutput, types::SdkError,
};
use howitt::checkpoint::Checkpoint;
use howitt::repo::Repo;
use howitt::route::Route;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct Keys {
    pk: String,
    sk: String,
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
        })
    }
}

#[derive(Debug, Constructor, Clone)]
pub struct SingleTableClient {
    client: dynamodb::Client,
    table_name: String,
}
impl SingleTableClient {
    pub async fn new_from_env() -> SingleTableClient {
        let config = aws_config::load_from_env().await;
        SingleTableClient {
            client: dynamodb::Client::new(&config),
            table_name: std::env::var("HOWITT_TABLE_NAME").unwrap_or("howitt".to_string()),
        }
    }

    pub async fn get(&self, keys: Keys) -> Result<GetItemOutput, SdkError<GetItemError>> {
        self.client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(keys.pk))
            .key("sk", AttributeValue::S(keys.sk))
            .send()
            .await
    }

    pub async fn put(
        &self,
        keys: Keys,
        item: HashMap<String, AttributeValue>,
    ) -> Result<PutItemOutput, SdkError<PutItemError>> {
        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .item("pk", AttributeValue::S(keys.pk))
            .item("sk", AttributeValue::S(keys.sk))
            .send()
            .await
    }

    pub async fn scan(&self) -> Result<ScanOutput, SdkError<ScanError>> {
        self.client.scan().table_name(&self.table_name).send().await
    }
}

#[async_trait::async_trait]
pub trait DynamoRepo<T: Send + Sync + Serialize + DeserializeOwned> {
    fn client(&self) -> &SingleTableClient;

    fn is_item(keys: &Keys) -> bool;

    fn keys(item: &T) -> Keys;

    async fn get(&self, id: String) -> Result<T, anyhow::Error> {
        let item = self.client().get(Keys::new(id.clone(), id.clone())).await?;
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

    async fn all(&self) -> Result<Vec<T>, anyhow::Error> {
        let scan_output = self.client().scan().await?;
        let items = scan_output.items().unwrap_or_default();

        Ok(items
            .into_iter()
            .filter(|item| match Keys::from_item(item) {
                Ok(keys) => Self::is_item(&keys),
                Err(_) => false,
            })
            .cloned()
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
        Keys::new(
            format!("ROUTE#{}", item.id),
            format!("ROUTE#{}", item.id),
        )
    }
}

#[async_trait::async_trait]
impl Repo<Route, anyhow::Error> for RouteRepo {
    async fn all(&self) -> Result<Vec<Route>, anyhow::Error> {
        DynamoRepo::all(self).await
    }
}
