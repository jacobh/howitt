use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use tokio_postgres::Client;

mod repos;
pub use repos::*;

/// A request-scoped connection on Workers. The lock prevents concurrent repository
/// calls from interleaving statements inside an explicit transaction.
#[derive(Clone)]
pub struct PostgresClient {
    client: Arc<Mutex<Client>>,
    #[cfg(not(target_arch = "wasm32"))]
    config: Option<Arc<tokio_postgres::Config>>,
}

impl std::fmt::Debug for PostgresClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Connection configuration includes credentials; never include it in logs.
        f.debug_struct("PostgresClient").finish_non_exhaustive()
    }
}

impl PostgresClient {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            #[cfg(not(target_arch = "wasm32"))]
            config: None,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect(url: &str) -> Result<Self, PostgresRepoError> {
        let config = Arc::new(url.parse::<tokio_postgres::Config>()?);
        let client = Self::connect_native(&config).await?;
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            config: Some(config),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn connect_native(config: &tokio_postgres::Config) -> Result<Client, PostgresRepoError> {
        let tls = postgres_native_tls::MakeTlsConnector::new(native_tls::TlsConnector::new()?);
        let (client, connection) = config.connect(tls).await?;
        tokio::spawn(async move {
            if let Err(error) = connection.await {
                tracing::error!(%error, "PostgreSQL connection closed");
            }
        });
        Ok(client)
    }

    pub async fn acquire(&self) -> Result<MutexGuard<'_, Client>, PostgresRepoError> {
        let client = self.client.lock().await;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut client = client;
            if client.is_closed() {
                if let Some(config) = &self.config {
                    // Reconnect for a new operation only. Never replay a failed
                    // statement/transaction, whose commit outcome may be unknown.
                    *client = Self::connect_native(config).await?;
                }
            }
            return Ok(client);
        }
        #[cfg(target_arch = "wasm32")]
        Ok(client)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PostgresRepoError {
    #[error("PostgreSQL query failed: {0}")]
    Postgres(#[from] tokio_postgres::Error),
    #[error("Invalid stored JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("PostgreSQL TLS configuration failed: {0}")]
    Tls(#[from] native_tls::Error),
}
