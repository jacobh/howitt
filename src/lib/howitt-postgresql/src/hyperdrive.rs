use crate::{ConnectionFactory, PostgresPool, PostgresRepoError};
use worker::{postgres_tls::PassthroughTls, Hyperdrive, SecureTransport, Socket};

struct HyperdriveConnectionFactory {
    host: String,
    port: u16,
    config: tokio_postgres::Config,
}

impl PostgresPool {
    /// Scope the resulting lazy pool to a fetch or queue invocation.
    pub fn from_hyperdrive(hyperdrive: Hyperdrive) -> Result<Self, PostgresRepoError> {
        Ok(Self::new(HyperdriveConnectionFactory {
            host: hyperdrive.host(),
            port: hyperdrive.port(),
            config: hyperdrive.connection_string().parse()?,
        }))
    }
}

#[async_trait::async_trait]
impl ConnectionFactory for HyperdriveConnectionFactory {
    async fn connect(&self) -> Result<tokio_postgres::Client, PostgresRepoError> {
        worker::send::SendFuture::new(async {
            let socket = Socket::builder()
                .secure_transport(SecureTransport::StartTls)
                .connect(&self.host, self.port)
                .map_err(|error| PostgresRepoError::Connection(error.to_string()))?;
            let (client, connection) = self.config.connect_raw(socket, PassthroughTls).await?;
            howitt_observability::spawn_local(async move {
                if connection.await.is_err() {
                    worker::console_error!("PostgreSQL connection closed with an error");
                }
            });
            Ok(client)
        })
        .await
    }
}
