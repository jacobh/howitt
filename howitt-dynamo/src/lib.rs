use std::collections::HashMap;
use std::str::FromStr;

use aws_sdk_dynamodb as dynamodb;
use derive_more::Constructor;
use dynamodb::error::{PutItemError, ScanError};
use dynamodb::output::{PutItemOutput, ScanOutput};
use dynamodb::{
    error::GetItemError, model::AttributeValue, output::GetItemOutput, types::SdkError,
};
use howitt::checkpoint::{Checkpoint, CheckpointType};
use howitt::repo::Repo;

#[derive(Debug, Constructor)]
pub struct Keys {
    pk: String,
    sk: String,
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
        item: HashMap<impl Into<String>, AttributeValue>,
    ) -> Result<PutItemOutput, SdkError<PutItemError>> {
        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item.into_iter().map(|(k, v)| (k.into(), v)).collect()))
            .item("pk", AttributeValue::S(keys.pk))
            .item("sk", AttributeValue::S(keys.sk))
            .send()
            .await
    }

    pub async fn scan(&self) -> Result<ScanOutput, SdkError<ScanError>> {
        self.client.scan().table_name(&self.table_name).send().await
    }
}

#[derive(Debug, Constructor, Clone)]
pub struct CheckpointRepo {
    client: SingleTableClient,
}
impl CheckpointRepo {
    fn deserialize_item(
        item: HashMap<String, AttributeValue>,
    ) -> Result<Checkpoint, anyhow::Error> {
        Ok(Checkpoint {
            id: uuid::Uuid::parse_str(item.get("id").unwrap().as_s().unwrap())?,
            name: item.get("name").unwrap().as_s().unwrap().to_owned(),
            point: {
                let coords: Vec<f64> = item
                    .get("point")
                    .unwrap()
                    .as_ns()
                    .unwrap()
                    .into_iter()
                    .map(|s| s.parse())
                    .collect::<Result<_, _>>()?;
                geo::Point::<f64>::new(coords[0], coords[1])
            },
            checkpoint_type: CheckpointType::from_str(
                item.get("checkpoint_type").unwrap().as_s().unwrap(),
            )
            .unwrap(),
        })
    }

    fn serialize_item(item: Checkpoint) -> HashMap<&'static str, AttributeValue> {
        maplit::hashmap! {
            "id" => AttributeValue::S(item.id.hyphenated().to_string()),
            "name" => AttributeValue::S(item.name.clone()),
            "point" => AttributeValue::Ns(vec![item.point.x().to_string(), item.point.y().to_string()]),
            "checkpoint_type" => AttributeValue::S(item.checkpoint_type.to_string())
        }
    }

    fn keys(item: &Checkpoint) -> Keys {
        Keys::new(
            format!("CHECKPOINT#{}", item.id.hyphenated()),
            format!("CHECKPOINT#{}", item.id.hyphenated()),
        )
    }

    pub async fn get(&self, id: String) -> Result<Checkpoint, anyhow::Error> {
        let item = self.client.get(Keys::new(id.clone(), id.clone())).await?;
        CheckpointRepo::deserialize_item(item.item().unwrap().clone())
    }

    pub async fn put(&self, item: Checkpoint) -> Result<(), anyhow::Error> {
        self.client
            .put(
                CheckpointRepo::keys(&item),
                CheckpointRepo::serialize_item(item),
            )
            .await?;

        Ok(())
    }

    pub async fn all(&self) -> Result<Vec<Checkpoint>, anyhow::Error> {
        let scan_output = self.client.scan().await?;
        let items = scan_output.items().unwrap_or_default();

        Ok(items
            .into_iter()
            .filter(|item| {
                item.get("pk")
                    .and_then(|x| x.as_s().ok())
                    .map(|pk| pk.starts_with("CHECKPOINT#"))
                    .unwrap_or(false)
            })
            .cloned()
            .map(CheckpointRepo::deserialize_item)
            .collect::<Result<_, _>>()?)
    }
}

#[async_trait::async_trait]
impl Repo<Checkpoint, anyhow::Error> for CheckpointRepo {
    async fn all(&self) -> Result<Vec<Checkpoint>, anyhow::Error> {
        self.all().await
    }
}
