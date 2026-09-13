use howitt_client_types::CacheStore;
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

// Bump the namespace when serialization or derived-data algorithms change.
const CACHE_VERSION: &str = "derived-v1";
const CACHE_TTL: Duration = Duration::from_secs(60 * 60);

pub struct CacheFetcher<Cache: CacheStore> {
    pub cache: Cache,
}

impl<Cache: CacheStore> CacheFetcher<Cache> {
    pub fn new(cache: Cache) -> Self {
        Self { cache }
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
        if let Some(data) = self.get(key).await {
            return Ok(data);
        }
        let data = fetch_data().await?;
        self.put(key, &data).await;
        Ok(data)
    }

    /// Separate lookup/fill operations let batch consumers fetch all misses together.
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        if !Cache::ENABLED {
            return None;
        }
        let key = format!("{CACHE_VERSION}:{key}");
        match self.cache.get_bytes(&key).await {
            Ok(Some(value)) => match bincode::deserialize(&value) {
                Ok(data) => return Some(data),
                Err(error) => tracing::warn!(%error, "Ignoring invalid derived cache value"),
            },
            Ok(None) => {}
            Err(error) => tracing::warn!(%error, "Derived cache read failed; computing directly"),
        }

        None
    }

    pub async fn put<T: Serialize>(&self, key: &str, data: &T) {
        if !Cache::ENABLED {
            return;
        }
        let key = format!("{CACHE_VERSION}:{key}");
        match bincode::serialize(data) {
            Ok(serialized) => {
                if let Err(error) = self
                    .cache
                    .set_bytes(&key, serialized.into(), CACHE_TTL)
                    .await
                {
                    tracing::warn!(%error, "Derived cache write failed; returning computed value");
                }
            }
            Err(error) => tracing::warn!(%error, "Derived cache serialization failed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct DisabledCache;
    #[async_trait::async_trait]
    impl CacheStore for DisabledCache {
        const ENABLED: bool = false;
        type Error = std::convert::Infallible;
        async fn get_bytes(&self, _: &str) -> Result<Option<bytes::Bytes>, Self::Error> {
            panic!("disabled cache must not be read")
        }
        async fn set_bytes(
            &self,
            _: &str,
            _: bytes::Bytes,
            _: Duration,
        ) -> Result<(), Self::Error> {
            panic!("disabled cache must not be written")
        }
    }
    #[derive(Default)]
    struct TestCache {
        value: Option<bytes::Bytes>,
        fail_read: bool,
        fail_write: bool,
        writes: std::sync::Mutex<Vec<(String, bytes::Bytes, Duration)>>,
    }

    #[async_trait::async_trait]
    impl CacheStore for TestCache {
        type Error = std::io::Error;

        async fn get_bytes(&self, key: &str) -> Result<Option<bytes::Bytes>, Self::Error> {
            assert_eq!(key, "derived-v1:example");
            if self.fail_read {
                return Err(std::io::Error::other("cache unavailable"));
            }
            Ok(self.value.clone())
        }

        async fn set_bytes(
            &self,
            key: &str,
            value: bytes::Bytes,
            ttl: Duration,
        ) -> Result<(), Self::Error> {
            self.writes.lock().unwrap().push((key.into(), value, ttl));
            if self.fail_write {
                return Err(std::io::Error::other("same-key write rate exceeded"));
            }
            Ok(())
        }
    }

    #[test]
    fn hit_skips_computation_and_write() {
        futures::executor::block_on(async {
            let fetcher = CacheFetcher::new(TestCache {
                value: Some(bincode::serialize(&42_u64).unwrap().into()),
                ..TestCache::default()
            });
            let value: u64 = fetcher
                .fetch_or_insert_with("example", || async { panic!("cache hit must not compute") })
                .await
                .unwrap();
            assert_eq!(value, 42);
            assert!(fetcher.cache.writes.lock().unwrap().is_empty());
        });
    }

    #[test_case::test_case(false, false, None; "miss")]
    #[test_case::test_case(true, false, None; "read failure")]
    #[test_case::test_case(false, true, None; "write failure")]
    #[test_case::test_case(false, false, Some(bytes::Bytes::from_static(b"bad")); "corrupt value")]
    fn computes_on_miss_or_cache_failure(
        fail_read: bool,
        fail_write: bool,
        value: Option<bytes::Bytes>,
    ) {
        futures::executor::block_on(async {
            let fetcher = CacheFetcher::new(TestCache {
                value,
                fail_read,
                fail_write,
                ..TestCache::default()
            });
            let value = fetcher
                .fetch_or_insert_with("example", || async { Ok(42_u64) })
                .await
                .unwrap();
            assert_eq!(value, 42);
            let writes = fetcher.cache.writes.lock().unwrap();
            assert_eq!(writes.len(), 1);
            assert_eq!(writes[0].0, "derived-v1:example");
            assert_eq!(bincode::deserialize::<u64>(&writes[0].1).unwrap(), 42);
            assert_eq!(writes[0].2, Duration::from_secs(3600));
        });
    }

    #[test]
    fn computation_errors_are_propagated_not_cached() {
        futures::executor::block_on(async {
            let fetcher = CacheFetcher::new(TestCache::default());
            let error = fetcher
                .fetch_or_insert_with::<u64, _, _>("example", || async {
                    Err(anyhow::anyhow!("source unavailable"))
                })
                .await
                .unwrap_err();
            assert_eq!(error.to_string(), "source unavailable");
            assert!(fetcher.cache.writes.lock().unwrap().is_empty());
        });
    }

    #[test]
    fn serialization_failure_does_not_fail_request() {
        futures::executor::block_on(async {
            let fetcher = CacheFetcher::new(TestCache::default());
            fetcher
                .fetch_or_insert_with("example", || async { Ok(NonSerializable) })
                .await
                .unwrap();
            assert!(fetcher.cache.writes.lock().unwrap().is_empty());
        });
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
