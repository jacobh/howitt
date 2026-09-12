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
        if !Redis::ENABLED {
            return fetch_data().await;
        }

        if let Some(value) = self.redis_client.get_bytes(key).await? {
            return Ok(bincode::deserialize(&value)?);
        }

        let data = fetch_data().await?;
        let serialized = bincode::serialize(&data)?;
        self.redis_client.set_bytes(key, serialized.into()).await?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct DisabledCache;
    #[async_trait::async_trait]
    impl howitt_client_types::RedisClient for DisabledCache {
        const ENABLED: bool = false;
        type Error = std::convert::Infallible;
        async fn get_bytes(&self, _: &str) -> Result<Option<bytes::Bytes>, Self::Error> {
            panic!("disabled cache must not be read")
        }
        async fn set_bytes(&self, _: &str, _: bytes::Bytes) -> Result<(), Self::Error> {
            panic!("disabled cache must not be written")
        }
    }
    #[derive(serde::Deserialize)]
    struct NonSerializable;
    impl serde::Serialize for NonSerializable {
        fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom(
                "uncached values must not be serialized",
            ))
        }
    }
    #[test]
    fn bypasses_cache_and_serialization() {
        futures::executor::block_on(async {
            CacheFetcher::new(DisabledCache)
                .fetch_or_insert_with("ignored", || async { Ok(NonSerializable) })
                .await
                .unwrap();
        });
    }
}
