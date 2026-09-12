//! Central statement instrumentation; no SQL text, parameters or error messages.
use howitt_observability::TraceSpan;
use std::ops::Deref;
use tokio::sync::MutexGuard;
use tokio_postgres::{
    types::{ToSql, Type},
    Client, Error, GenericClient, Row, Transaction,
};

pub struct TracedClient<C> {
    inner: C,
}
pub type PostgresConnection<'a> = TracedClient<MutexGuard<'a, Client>>;
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
    pub(crate) fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C: Deref> TracedClient<C>
where
    C::Target: GenericClient,
{
    pub async fn query_typed(
        &self,
        statement: &str,
        params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Vec<Row>, Error> {
        db_span("query_typed", operation(statement))
            .trace(self.inner.query_typed(statement, params))
            .await
    }
    pub async fn query_typed_one(
        &self,
        statement: &str,
        params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Row, Error> {
        db_span("query_typed_one", operation(statement))
            .trace(self.inner.query_typed_one(statement, params))
            .await
    }
    pub async fn query_typed_opt(
        &self,
        statement: &str,
        params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<Option<Row>, Error> {
        db_span("query_typed_opt", operation(statement))
            .trace(self.inner.query_typed_opt(statement, params))
            .await
    }
    pub async fn execute_typed(
        &self,
        statement: &str,
        params: &[(&(dyn ToSql + Sync), Type)],
    ) -> Result<u64, Error> {
        db_span("execute_typed", operation(statement))
            .trace(self.inner.execute_typed(statement, params))
            .await
    }
}

impl PostgresConnection<'_> {
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    pub async fn transaction(&mut self) -> Result<PostgresTransaction<'_>, Error> {
        let transaction = db_span("transaction.begin", "BEGIN")
            .trace(self.inner.transaction())
            .await?;
        Ok(TracedClient::new(Box::new(transaction)))
    }
}

impl PostgresTransaction<'_> {
    pub async fn commit(self) -> Result<(), Error> {
        db_span("transaction.commit", "COMMIT")
            .trace(self.inner.commit())
            .await
    }
    pub async fn rollback(self) -> Result<(), Error> {
        db_span("transaction.rollback", "ROLLBACK")
            .trace(self.inner.rollback())
            .await
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
