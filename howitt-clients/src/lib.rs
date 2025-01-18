use async_once_cell::OnceCell;

use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    operation::{head_object::HeadObjectError, put_object::PutObjectError},
    primitives::ByteStream,
};
use derive_more::derive::Constructor;
use howitt_client_types::{BucketClient, BucketName, HttpClient, HttpResponse};
use redis::{AsyncCommands, IntoConnectionInfo};

#[derive(thiserror::Error, Debug, derive_more::Display)]
pub enum S3BucketClientError {
    HeadError(#[from] HeadObjectError),
    PutError(#[from] PutObjectError),
}

#[derive(derive_more::Constructor, Debug)]
pub struct S3BucketClient {
    client: aws_sdk_s3::Client,
    bucket_name: BucketName,
}

impl S3BucketClient {
    pub async fn new_from_env(bucket_name: BucketName) -> S3BucketClient {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        S3BucketClient {
            client: aws_sdk_s3::Client::new(&config),
            bucket_name: bucket_name,
        }
    }
}

#[async_trait]
impl BucketClient for S3BucketClient {
    type Error = S3BucketClientError;

    async fn key_exists(&self, key: &str) -> Result<bool, Self::Error> {
        let result = self
            .client
            .head_object()
            .bucket(self.bucket_name.to_bucket_name())
            .key(key)
            .send()
            .await
            .map_err(|e| e.into_service_error());

        match result {
            Ok(_) => Ok(true),
            Err(HeadObjectError::NotFound(_)) => Ok(false),
            Err(e) => Err(S3BucketClientError::HeadError(e)),
        }
    }

    async fn put_object(&self, key: &str, body: bytes::Bytes) -> Result<(), Self::Error> {
        self.client
            .put_object()
            .bucket(self.bucket_name.to_bucket_name())
            .key(key)
            .body(ByteStream::from(body))
            .send()
            .await
            .map_err(|e| e.into_service_error())?;

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
