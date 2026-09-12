use axum::body::Bytes;

/// Request-scoped binding: never retain Workers I/O handles across requests.
#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct WorkerCache(worker::send::SendWrapper<worker::kv::KvStore>);

#[cfg(target_arch = "wasm32")]
impl WorkerCache {
    pub fn new(store: worker::kv::KvStore) -> Self {
        Self(worker::send::SendWrapper::new(store))
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait]
impl howitt_client_types::CacheStore for WorkerCache {
    type Error = std::io::Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Bytes>, Self::Error> {
        worker::send::SendFuture::new(async {
            self.0
                .get(key)
                .bytes()
                .await
                .map(|value| value.map(Bytes::from))
                .map_err(|error| std::io::Error::other(error.to_string()))
        })
        .await
    }

    async fn set_bytes(
        &self,
        key: &str,
        bytes: Bytes,
        ttl: std::time::Duration,
    ) -> Result<(), Self::Error> {
        worker::send::SendFuture::new(async {
            self.0
                .put_bytes(key, &bytes)
                .map_err(|error| std::io::Error::other(error.to_string()))?
                .expiration_ttl(ttl.as_secs())
                .execute()
                .await
                .map_err(|error| std::io::Error::other(error.to_string()))
        })
        .await
    }
}

// The Worker schema also compiles on the host for unit tests, where JS bindings
// cannot run. Native applications continue to use the real Redis CacheStore.
#[cfg(not(target_arch = "wasm32"))]
pub struct WorkerCache;

#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
impl howitt_client_types::CacheStore for WorkerCache {
    const ENABLED: bool = false;
    type Error = std::convert::Infallible;

    async fn get_bytes(&self, _: &str) -> Result<Option<Bytes>, Self::Error> {
        Ok(None)
    }

    async fn set_bytes(
        &self,
        _: &str,
        _: Bytes,
        _: std::time::Duration,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
