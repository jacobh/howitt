use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres};

mod repos;

pub use repos::*;

#[derive(Debug, Clone)]
pub struct PostgresClient {
    pool: PgPool,
}

impl PostgresClient {
    pub async fn connect(url: &str) -> Result<PostgresClient, PostgresRepoError> {
        Ok(PostgresClient {
            pool: PgPoolOptions::new()
                .max_connections(10)
                .connect(url)
                .await?,
        })
    }

    async fn acquire(&self) -> Result<PoolConnection<Postgres>, PostgresRepoError> {
        Ok(self.pool.acquire().await?)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Postgres Repo Error {:?}", _0)]
pub enum PostgresRepoError {
    Sqlx(#[from] sqlx::Error),
    SerdeJson(#[from] serde_json::Error),
}
