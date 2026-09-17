//! Central statement instrumentation; no SQL text, parameters or error messages.
use crate::pool::ConnectionLease;
use howitt_observability::TraceSpan;
use std::{
    ops::Deref,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio_postgres::{
    Error, GenericClient, Row, Transaction,
    types::{ToSql, Type},
};

pub struct TracedClient<C> {
    inner: C,
    reusable: Arc<AtomicBool>,
}
pub type PostgresConnection = TracedClient<ConnectionLease>;
pub type PostgresTransaction<'a> = TracedClient<Box<Transaction<'a>>>;

pub(crate) fn db_span(method: &str, operation: &str) -> TraceSpan {
    let span = TraceSpan::new(&format!("postgres.{method}"), "db.outcome");
    span.attribute("db.system.name", "postgresql");
    span.attribute("db.operation.name", operation);
    span
}

// A bounded classification, never a substring of caller-supplied SQL in traces.
fn operation(statement: &str) -> &'static str {
    match statement
        .split_ascii_whitespace()
        .next()
        .map(str::to_ascii_uppercase)
        .as_deref()
    {
        Some("SELECT") => "SELECT",
        Some("INSERT") => "INSERT",
        Some("UPDATE") => "UPDATE",
        Some("DELETE") => "DELETE",
        Some("WITH") => "WITH",
        _ => "OTHER",
    }
}

impl<C> TracedClient<C> {
    fn new(inner: C, reusable: Arc<AtomicBool>) -> Self {
        Self { inner, reusable }
    }

    fn check_connection<T>(&self, result: Result<T, Error>) -> Result<T, Error> {
        // A server error can precede the driver's is_closed update. Discard on
        // failure rather than leasing a dying connection or replaying a query.
        result.inspect_err(|_| self.reusable.store(false, Ordering::Relaxed))
    }
}

impl<C: Deref> TracedClient<C>
where
    C::Target: GenericClient,
{
    pub async fn batch_execute(&self, statement: &str) -> Result<(), Error> {
        self.check_connection(
            db_span("batch_execute", operation(statement))
                .trace(self.inner.batch_execute(statement))
                .await,
        )
    }

    pub async fn query_typed(
        &self,
        statement: &str,
        params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Vec<Row>, Error> {
        self.check_connection(
            db_span("query_typed", operation(statement))
                .trace(self.inner.query_typed(statement, params))
                .await,
        )
    }
    pub async fn query_typed_one(
        &self,
        statement: &str,
        params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Row, Error> {
        self.check_connection(
            db_span("query_typed_one", operation(statement))
                .trace(self.inner.query_typed_one(statement, params))
                .await,
        )
    }
    pub async fn query_typed_opt(
        &self,
        statement: &str,
        params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Option<Row>, Error> {
        self.check_connection(
            db_span("query_typed_opt", operation(statement))
                .trace(self.inner.query_typed_opt(statement, params))
                .await,
        )
    }
    pub async fn execute_typed(
        &self,
        statement: &str,
        params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<u64, Error> {
        self.check_connection(
            db_span("execute_typed", operation(statement))
                .trace(self.inner.execute_typed(statement, params))
                .await,
        )
    }
}

impl PostgresConnection {
    pub(crate) fn from_lease(inner: ConnectionLease) -> Self {
        let reusable = inner.reusable.clone();
        Self::new(inner, reusable)
    }

    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    pub async fn transaction(&mut self) -> Result<PostgresTransaction<'_>, Error> {
        // BEGIN failure/cancellation and dropped transactions discard the connection.
        // Only an acknowledged COMMIT/ROLLBACK makes it safe to pool again.
        self.reusable.store(false, Ordering::Relaxed);
        let transaction = db_span("transaction.begin", "BEGIN")
            .trace(self.inner.transaction())
            .await?;
        Ok(TracedClient::new(
            Box::new(transaction),
            self.reusable.clone(),
        ))
    }
}

impl PostgresTransaction<'_> {
    pub async fn commit(self) -> Result<(), Error> {
        db_span("transaction.commit", "COMMIT")
            .trace(self.inner.commit())
            .await?;
        self.reusable.store(true, Ordering::Relaxed);
        Ok(())
    }
    pub async fn rollback(self) -> Result<(), Error> {
        db_span("transaction.rollback", "ROLLBACK")
            .trace(self.inner.rollback())
            .await?;
        self.reusable.store(true, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test_case::test_case(" select secret from users", "SELECT")]
    #[test_case::test_case("INSERT INTO users VALUES ('secret')", "INSERT")]
    #[test_case::test_case("update users set password='secret'", "UPDATE")]
    #[test_case::test_case("delete from users", "DELETE")]
    #[test_case::test_case("WITH x AS (SELECT 1) SELECT * FROM x", "WITH")]
    #[test_case::test_case("/* secret */ select 1", "OTHER")]
    #[test_case::test_case("secret", "OTHER")]
    #[test_case::test_case("", "OTHER")]
    fn statement_metadata_is_bounded(statement: &str, expected: &str) {
        assert_eq!(operation(statement), expected);
    }
}
