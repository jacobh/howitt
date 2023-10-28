use async_trait::async_trait;
use aws_sdk_s3::{operation::head_object::HeadObjectError, primitives::ByteStream};
use howitt_client_types::{BucketClient, HttpClient, HttpResponse};

#[derive(thiserror::Error, Debug, derive_more::Display)]
pub struct S3BucketClientError;

#[derive(derive_more::Constructor)]
pub struct S3BucketClient {
    client: aws_sdk_s3::Client,
    bucket_name: String,
}

#[async_trait]
impl BucketClient for S3BucketClient {
    type Error = S3BucketClientError;

    async fn key_exists(&self, key: &str) -> Result<bool, Self::Error> {
        let result = self
            .client
            .head_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| e.into_service_error());

        match result {
            Ok(_) => Ok(true),
            Err(HeadObjectError::NotFound(_)) => Ok(false),
            Err(_) => Err(S3BucketClientError {}),
        }
    }

    async fn put_object(&self, key: &str, body: bytes::Bytes) -> Result<(), Self::Error> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(body))
            .send()
            .await
            .map_err(|_| S3BucketClientError)?;

        Ok(())
    }
}

#[derive(derive_more::Constructor)]
pub struct ReqwestHttpClient {
    client: reqwest::Client,
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
