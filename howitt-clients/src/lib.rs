use async_trait::async_trait;
use howitt_client_types::{BucketClient, BucketName, HttpClient, HttpResponse};
use object_store::{aws::AmazonS3, ObjectStore};
use redis::{AsyncCommands, IntoConnectionInfo};

#[derive(derive_more::Constructor, Debug)]
pub struct S3BucketClient {
    client: AmazonS3,
}

impl S3BucketClient {
    pub fn new_from_env(bucket_name: BucketName) -> S3BucketClient {
        let client = object_store::aws::AmazonS3Builder::from_env()
            .with_region("ap-southeast-4")
            .with_bucket_name(bucket_name.to_bucket_name())
            .build()
            .unwrap();

        S3BucketClient { client }
    }
}

#[async_trait]
impl BucketClient for S3BucketClient {
    type Error = object_store::Error;

    async fn key_exists(&self, key: &str) -> Result<bool, Self::Error> {
        match self.client.head(&key.into()).await {
            Ok(_) => Ok(true),
            Err(object_store::Error::NotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn put_object(&self, key: &str, body: bytes::Bytes) -> Result<(), Self::Error> {
        self.client.put(&key.into(), body.into()).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ReqwestHttpClient {
    client: reqwest::Client,
}

impl ReqwestHttpClient {
    pub fn new() -> ReqwestHttpClient {
        ReqwestHttpClient {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    type Error = reqwest::Error;

    async fn get(&self, url: url::Url) -> Result<HttpResponse, Self::Error> {
        let resp = self.client.get(url).send().await?;

        Ok(HttpResponse {
            body: resp.bytes().await?,
        })
    }
}

#[derive(Debug)]
pub struct RedisClient {
    pub client: redis::Client,
    pub conn: redis::aio::MultiplexedConnection,
}

impl RedisClient {
    pub async fn connect(
        connection_info: impl IntoConnectionInfo,
    ) -> Result<RedisClient, redis::RedisError> {
        let client = redis::Client::open(connection_info)?;
        let conn = client.get_multiplexed_async_connection().await?;

        Ok(RedisClient { client, conn })
    }

    fn conn(&self) -> redis::aio::MultiplexedConnection {
        self.conn.clone()
    }
}

#[async_trait::async_trait]
impl howitt_client_types::RedisClient for RedisClient {
    type Error = redis::RedisError;

    async fn get_bytes(&self, key: &str) -> Result<Option<bytes::Bytes>, Self::Error> {
        Ok(self.conn().get(key).await?)
    }
    async fn set_bytes(&self, key: &str, bytes: bytes::Bytes) -> Result<(), Self::Error> {
        Ok(self.conn().set(key, bytes.to_vec()).await?)
    }
}
