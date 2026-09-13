use howitt_postgresql::{PostgresConnection, PostgresPool};
use std::{collections::HashSet, time::Duration};

type TestResult = Result<(), Box<dyn std::error::Error>>;

async fn pool() -> Result<PostgresPool, Box<dyn std::error::Error>> {
    let url = std::env::var("HOWITT_TEST_DATABASE_URL")?;
    let config: tokio_postgres::Config = url.parse()?;
    assert!(
        config
            .get_dbname()
            .is_some_and(|name| name.starts_with("howitt_workers_test_"))
    );
    assert!(
        matches!(config.get_hosts(), [tokio_postgres::config::Host::Tcp(host)] if host == "127.0.0.1")
    );
    Ok(PostgresPool::connect(&url).await?)
}

async fn pid(conn: &PostgresConnection) -> Result<i32, tokio_postgres::Error> {
    Ok(conn
        .query_typed_one("SELECT pg_backend_pid()", &[])
        .await?
        .get(0))
}

#[tokio::test]
#[ignore = "requires scripts/test-worker-local.sh disposable database"]
async fn dropping_pool_closes_idle_connections() -> TestResult {
    let observer_pool = pool().await?;
    let observer = observer_pool.acquire().await?;
    let scoped_pool = pool().await?;
    let conn = scoped_pool.acquire().await?;
    let scoped_pid = pid(&conn).await?;
    drop(conn);
    drop(scoped_pool);
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let active: bool = observer
                .query_typed_one(
                    "SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE pid = $1)",
                    &[(&scoped_pid, tokio_postgres::types::Type::INT4)],
                )
                .await?
                .get(0);
            if !active {
                return Ok::<_, tokio_postgres::Error>(());
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await??;
    Ok(())
}

#[tokio::test]
#[ignore = "requires scripts/test-worker-local.sh disposable database"]
async fn five_connections_are_exclusive_and_reused() -> TestResult {
    let pool = pool().await?;
    let mut leases = Vec::new();
    let mut pids = HashSet::new();
    for _ in 0..5 {
        let conn = pool.acquire().await?;
        pids.insert(pid(&conn).await?);
        leases.push(conn);
    }
    assert_eq!(pids.len(), 5);
    assert!(
        tokio::time::timeout(Duration::from_millis(50), pool.acquire())
            .await
            .is_err()
    );
    let released = leases.pop().unwrap();
    let released_pid = pid(&released).await?;
    drop(released);
    let conn = tokio::time::timeout(Duration::from_secs(2), pool.acquire()).await??;
    assert_eq!(pid(&conn).await?, released_pid);
    Ok(())
}

#[tokio::test]
#[ignore = "requires scripts/test-worker-local.sh disposable database"]
async fn queries_run_while_another_connection_is_busy() -> TestResult {
    let pool = pool().await?;
    let slow = pool.acquire().await?;
    let fast = pool.acquire().await?;
    let (slow_result, fast_result) = tokio::join!(
        slow.query_typed_one("SELECT pg_sleep(0.5)", &[]),
        tokio::time::timeout(
            Duration::from_millis(250),
            fast.query_typed_one("SELECT 1", &[])
        ),
    );
    slow_result?;
    assert_eq!(fast_result??.get::<_, i32>(0), 1);
    Ok(())
}

#[tokio::test]
#[ignore = "requires scripts/test-worker-local.sh disposable database"]
async fn transactions_are_isolated_and_clean_commits_are_reused() -> TestResult {
    let pool = pool().await?;
    let mut conn = pool.acquire().await?;
    let original_pid = pid(&conn).await?;
    let tx = conn.transaction().await?;
    tx.execute_typed("SET LOCAL application_name = 'pool-transaction-test'", &[])
        .await?;
    let other = pool.acquire().await?;
    let name: String = other
        .query_typed_one("SELECT current_setting('application_name')", &[])
        .await?
        .get(0);
    assert_ne!(name, "pool-transaction-test");
    tx.commit().await?;
    drop(conn);
    let mut reused = pool.acquire().await?;
    assert_eq!(pid(&reused).await?, original_pid);
    reused.transaction().await?.rollback().await?;
    drop(reused);
    assert_eq!(pid(&pool.acquire().await?).await?, original_pid);
    Ok(())
}

#[tokio::test]
#[ignore = "requires scripts/test-worker-local.sh disposable database"]
async fn dropped_and_failed_transactions_are_discarded() -> TestResult {
    let pool = pool().await?;
    let mut conn = pool.acquire().await?;
    let original_pid = pid(&conn).await?;
    let tx = conn.transaction().await?;
    assert!(tx.query_typed_one("SELECT 1 / 0", &[]).await.is_err());
    drop(tx);
    drop(conn);
    let mut conn = pool.acquire().await?;
    let replacement_pid = pid(&conn).await?;
    assert_ne!(replacement_pid, original_pid);
    let tx = conn.transaction().await?;
    drop(tx);
    drop(conn);
    assert_ne!(pid(&pool.acquire().await?).await?, replacement_pid);
    Ok(())
}

#[tokio::test]
#[ignore = "requires scripts/test-worker-local.sh disposable database"]
async fn cancelled_begin_and_commit_discard_connections() -> TestResult {
    let pool = pool().await?;
    let mut conn = pool.acquire().await?;
    let original_pid = pid(&conn).await?;
    {
        let begin = conn.transaction();
        tokio::pin!(begin);
        assert!(futures_util::poll!(&mut begin).is_pending());
    }
    drop(conn);
    let mut conn = pool.acquire().await?;
    let replacement_pid = pid(&conn).await?;
    assert_ne!(replacement_pid, original_pid);
    let tx = conn.transaction().await?;
    {
        let commit = tx.commit();
        tokio::pin!(commit);
        assert!(futures_util::poll!(&mut commit).is_pending());
    }
    drop(conn);
    assert_ne!(pid(&pool.acquire().await?).await?, replacement_pid);
    Ok(())
}

#[tokio::test]
#[ignore = "requires scripts/test-worker-local.sh disposable database"]
async fn closed_connections_are_replaced_without_replaying_statements() -> TestResult {
    let pool = pool().await?;
    let conn = pool.acquire().await?;
    let original_pid = pid(&conn).await?;
    assert!(
        conn.query_typed_one("SELECT pg_terminate_backend(pg_backend_pid())", &[])
            .await
            .is_err()
    );
    drop(conn);
    assert_ne!(pid(&pool.acquire().await?).await?, original_pid);
    Ok(())
}
