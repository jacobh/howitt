/// The first Worker release computes derived values directly. This adapter keeps
/// the shared native fetchers usable without a Redis dependency or cache writes.
#[derive(Clone, Copy)]
pub struct Uncached;

#[async_trait::async_trait]
impl howitt_client_types::RedisClient for Uncached {
    const ENABLED: bool = false;

    type Error = std::convert::Infallible;

    async fn get_bytes(&self, _key: &str) -> Result<Option<axum::body::Bytes>, Self::Error> {
        Ok(None)
    }

    async fn set_bytes(&self, _key: &str, _bytes: axum::body::Bytes) -> Result<(), Self::Error> {
        Ok(())
    }
}
