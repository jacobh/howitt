#[cfg(target_arch = "wasm32")]
mod hyperdrive;
mod pool;
mod repos;
mod rwgps_sync;
mod traced_client;
pub use pool::{ConnectionFactory, PostgresPool};
pub use repos::*;
pub use rwgps_sync::PostgresRwgpsSyncStore;
pub use traced_client::{PostgresConnection, PostgresTransaction};

impl PostgresPool {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect(url: &str) -> Result<Self, PostgresRepoError> {
        Ok(Self::new(NativeConnectionFactory(url.parse()?)))
    }

    pub async fn acquire(&self) -> Result<PostgresConnection, PostgresRepoError> {
        let lease = traced_client::db_span("connection.wait", "ACQUIRE")
            .trace(self.lease())
            .await?;
        Ok(PostgresConnection::from_lease(lease))
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeConnectionFactory(tokio_postgres::Config);

#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
impl ConnectionFactory for NativeConnectionFactory {
    async fn connect(&self) -> Result<tokio_postgres::Client, PostgresRepoError> {
        let tls = postgres_native_tls::MakeTlsConnector::new(native_tls::TlsConnector::new()?);
        let (client, connection) = self.0.connect(tls).await?;
        tokio::spawn(async move {
            if let Err(error) = connection.await {
                tracing::error!(%error, "PostgreSQL connection closed");
            }
        });
        Ok(client)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PostgresRepoError {
    #[error("PostgreSQL connection acquisition timed out")]
    AcquireTimeout,
    #[error("PostgreSQL connection pool closed")]
    PoolClosed,
    #[error("PostgreSQL socket connection failed: {0}")]
    Connection(String),
    #[error("PostgreSQL query failed: {0}")]
    Postgres(#[from] tokio_postgres::Error),
    #[error("Invalid stored JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("PostgreSQL TLS configuration failed: {0}")]
    Tls(#[from] native_tls::Error),
}
