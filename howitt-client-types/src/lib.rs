use bytes::Bytes;

#[derive(Debug)]
pub enum BucketName {
    Photos,
}

impl BucketName {
    pub fn to_bucket_name(&self) -> &'static str {
        match self {
            BucketName::Photos => "howitt-photos",
        }
    }
}

#[async_trait::async_trait]
pub trait BucketClient {
    type Error;

    async fn key_exists(&self, key: &str) -> Result<bool, Self::Error>;

    async fn put_object(&self, key: &str, body: bytes::Bytes) -> Result<(), Self::Error>;
}

pub struct HttpResponse {
    pub body: Bytes,
}

#[async_trait::async_trait]
pub trait HttpClient {
    type Error;

    async fn get(&self, url: url::Url) -> Result<HttpResponse, Self::Error>;
}
