use bytes::Bytes;

#[derive(Debug)]
pub enum BucketName {
    Photos,
    Media,
}

impl BucketName {
    pub fn to_bucket_name(&self) -> &'static str {
        match self {
            BucketName::Photos => "howitt-photos",
            BucketName::Media => "howitt-media",
        }
    }
}

pub struct ObjectParams {
    pub content_type: Option<String>,
}

#[async_trait::async_trait]
pub trait BucketClient {
    type Error;

    async fn key_exists(&self, key: &str) -> Result<bool, Self::Error>;

    async fn put_object(
        &self,
        key: &str,
        body: bytes::Bytes,
        params: ObjectParams,
    ) -> Result<(), Self::Error>;

    async fn get_object(&self, key: &str) -> Result<Option<bytes::Bytes>, Self::Error>;
}

pub struct HttpResponse {
    pub body: Bytes,
}

#[async_trait::async_trait]
pub trait HttpClient {
    type Error;

    async fn get(&self, url: url::Url) -> Result<HttpResponse, Self::Error>;
}

#[async_trait::async_trait]
pub trait RedisClient {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn get_bytes(&self, key: &str) -> Result<Option<bytes::Bytes>, Self::Error>;
    async fn set_bytes(&self, key: &str, bytes: bytes::Bytes) -> Result<(), Self::Error>;
}
