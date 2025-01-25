use howitt_client_types::RedisClient;
use serde::{de::DeserializeOwned, Serialize};

pub struct CacheFetcher<Redis: RedisClient> {
    pub redis_client: Redis,
}

impl<Redis: RedisClient> CacheFetcher<Redis> {
    pub fn new(redis_client: Redis) -> Self {
        Self { redis_client }
    }

    pub async fn fetch_or_insert_with<T, F, Fut>(
        &self,
        key: &str,
        fetch_data: F,
    ) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned + Serialize,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, anyhow::Error>>,
    {
        if let Some(value) = self.redis_client.get_bytes(key).await? {
            return Ok(bincode::deserialize(&value)?);
        }

        let data = fetch_data().await?;
        let serialized = bincode::serialize(&data)?;
        self.redis_client.set_bytes(key, serialized.into()).await?;

        Ok(data)
    }
}
