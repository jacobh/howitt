use crate::{PostgresPool, PostgresRepoError};
use sha2::{Digest, Sha256};
use tokio_postgres::types::Type;

const INITIALIZE_AND_LOCK_HISTORY: &str = r#"
DO $migration_history$
BEGIN
    CREATE TABLE IF NOT EXISTS howitt_schema_migrations (
        version BIGINT PRIMARY KEY CHECK (version > 0),
        name TEXT NOT NULL,
        checksum TEXT NOT NULL CHECK (length(checksum) = 64),
        execution_mode TEXT NOT NULL CHECK (execution_mode IN ('applied', 'baseline')),
        applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );
EXCEPTION
    -- Concurrent first invocations can both observe the table as absent. The
    -- loser continues after the winner commits, then takes the table lock below.
    WHEN duplicate_table OR unique_violation THEN NULL;
END
$migration_history$;
LOCK TABLE howitt_schema_migrations IN EXCLUSIVE MODE
"#;

#[derive(Clone, Copy, Debug)]
pub struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/migrations.rs"));

pub fn bundled_migrations() -> &'static [Migration] {
    BUNDLED_MIGRATIONS
}

#[derive(Debug, PartialEq, Eq)]
pub struct MigrationResult {
    pub applied_versions: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BaselineResult {
    pub recorded_versions: Vec<i64>,
}

#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error(transparent)]
    Repository(#[from] PostgresRepoError),
    #[error("database migration failed during {phase}")]
    Database {
        phase: &'static str,
        version: Option<i64>,
        #[source]
        source: tokio_postgres::Error,
    },
    #[error("migration history contains unknown version V{0:04}")]
    UnknownVersion(i64),
    #[error("migration V{version:04} name differs from the bundled migration")]
    NameDrift { version: i64 },
    #[error("migration V{version:04} checksum differs from the bundled migration")]
    ChecksumDrift { version: i64 },
    #[error("bundled migrations are not strictly ordered at V{0:04}")]
    InvalidOrder(i64),
    #[error(
        "migration history is not an ordered prefix; expected V{expected:04}, found V{found:04}"
    )]
    HistoryGap { expected: i64, found: i64 },
    #[error("baseline requires an empty migration history")]
    BaselineHistoryNotEmpty,
    #[error("baseline target V{0:04} is not a bundled migration")]
    InvalidBaseline(i64),
}

impl MigrationError {
    fn database(
        phase: &'static str,
        version: Option<i64>,
    ) -> impl FnOnce(tokio_postgres::Error) -> Self {
        move |source| Self::Database {
            phase,
            version,
            source,
        }
    }

    pub fn database_diagnostic(&self) -> Option<(&'static str, Option<i64>, Option<&str>)> {
        let Self::Database {
            phase,
            version,
            source,
        } = self
        else {
            return None;
        };
        Some((
            phase,
            *version,
            source.as_db_error().map(|error| error.code().code()),
        ))
    }
}

fn checksum(sql: &str) -> String {
    format!("{:x}", Sha256::digest(sql.as_bytes()))
}

fn validate_bundle(migrations: &[Migration]) -> Result<(), MigrationError> {
    for pair in migrations.windows(2) {
        if pair[0].version >= pair[1].version {
            return Err(MigrationError::InvalidOrder(pair[1].version));
        }
    }
    Ok(())
}

async fn initialize_and_lock(
    transaction: &crate::PostgresTransaction<'_>,
) -> Result<(), tokio_postgres::Error> {
    transaction.batch_execute(INITIALIZE_AND_LOCK_HISTORY).await
}

async fn validate_history(
    transaction: &crate::PostgresTransaction<'_>,
    migrations: &[Migration],
) -> Result<usize, MigrationError> {
    let rows = transaction
        .query_typed(
            "SELECT version, name, checksum FROM howitt_schema_migrations ORDER BY version",
            &[],
        )
        .await
        .map_err(MigrationError::database("validate_history", None))?;
    for (index, row) in rows.iter().enumerate() {
        let version = row.get::<_, i64>(0);
        let migration = migrations
            .iter()
            .find(|migration| migration.version == version)
            .ok_or(MigrationError::UnknownVersion(version))?;
        if migrations[index].version != version {
            return Err(MigrationError::HistoryGap {
                expected: migrations[index].version,
                found: version,
            });
        }
        if row.get::<_, &str>(1) != migration.name {
            return Err(MigrationError::NameDrift { version });
        }
        if row.get::<_, &str>(2) != checksum(migration.sql) {
            return Err(MigrationError::ChecksumDrift { version });
        }
    }
    Ok(rows.len())
}

async fn record(
    transaction: &crate::PostgresTransaction<'_>,
    migration: &Migration,
    mode: &str,
) -> Result<(), tokio_postgres::Error> {
    let digest = checksum(migration.sql);
    transaction
        .execute_typed(
            "INSERT INTO howitt_schema_migrations (version, name, checksum, execution_mode) VALUES ($1, $2, $3, $4)",
            &[
                (&migration.version, Type::INT8),
                (&migration.name, Type::TEXT),
                (&digest, Type::TEXT),
                (&mode, Type::TEXT),
            ],
        )
        .await?;
    Ok(())
}

pub async fn run_migrations(
    pool: &PostgresPool,
    migrations: &[Migration],
) -> Result<MigrationResult, MigrationError> {
    validate_bundle(migrations)?;
    let mut connection = pool.acquire().await?;
    let transaction = connection
        .transaction()
        .await
        .map_err(MigrationError::database("begin", None))?;
    initialize_and_lock(&transaction)
        .await
        .map_err(MigrationError::database("initialize_lock", None))?;
    validate_history(&transaction, migrations).await?;

    let rows = transaction
        .query_typed("SELECT version FROM howitt_schema_migrations", &[])
        .await
        .map_err(MigrationError::database("read_history", None))?;
    let applied = rows
        .iter()
        .map(|row| row.get::<_, i64>(0))
        .collect::<std::collections::HashSet<_>>();
    let mut applied_versions = Vec::new();
    for migration in migrations {
        if !applied.contains(&migration.version) {
            transaction
                .batch_execute(migration.sql)
                .await
                .map_err(MigrationError::database("execute", Some(migration.version)))?;
            record(&transaction, migration, "applied")
                .await
                .map_err(MigrationError::database("record", Some(migration.version)))?;
            applied_versions.push(migration.version);
        }
    }
    transaction
        .commit()
        .await
        .map_err(MigrationError::database("commit", None))?;
    Ok(MigrationResult { applied_versions })
}

pub async fn baseline_migrations(
    pool: &PostgresPool,
    migrations: &[Migration],
    through: i64,
) -> Result<BaselineResult, MigrationError> {
    validate_bundle(migrations)?;
    if !migrations
        .iter()
        .any(|migration| migration.version == through)
    {
        return Err(MigrationError::InvalidBaseline(through));
    }
    let mut connection = pool.acquire().await?;
    let transaction = connection
        .transaction()
        .await
        .map_err(MigrationError::database("begin", None))?;
    initialize_and_lock(&transaction)
        .await
        .map_err(MigrationError::database("initialize_lock", None))?;
    if validate_history(&transaction, migrations).await? != 0 {
        return Err(MigrationError::BaselineHistoryNotEmpty);
    }

    let mut recorded_versions = Vec::new();
    for migration in migrations
        .iter()
        .filter(|migration| migration.version <= through)
    {
        record(&transaction, migration, "baseline")
            .await
            .map_err(MigrationError::database(
                "record_baseline",
                Some(migration.version),
            ))?;
        recorded_versions.push(migration.version);
    }
    transaction
        .commit()
        .await
        .map_err(MigrationError::database("commit", None))?;
    Ok(BaselineResult { recorded_versions })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_migrations_are_strictly_ordered() {
        assert!(
            bundled_migrations()
                .windows(2)
                .all(|pair| pair[0].version < pair[1].version)
        );
        assert_eq!(bundled_migrations().last().unwrap().version, 26);
    }

    #[test]
    fn checksum_is_stable_and_sensitive_to_content() {
        assert_eq!(
            checksum("SELECT 1;"),
            "17db4fd369edb9244b9f91d9aeed145c3d04ad8ba6e95d06247f07a63527d11a"
        );
        assert_ne!(checksum("SELECT 1;"), checksum("SELECT 1;\n"));
    }

    #[test]
    fn rejects_out_of_order_migrations() {
        let migrations = [
            Migration {
                version: 2,
                name: "second",
                sql: "SELECT 2",
            },
            Migration {
                version: 1,
                name: "first",
                sql: "SELECT 1",
            },
        ];
        assert!(matches!(
            validate_bundle(&migrations),
            Err(MigrationError::InvalidOrder(1))
        ));
    }
}
