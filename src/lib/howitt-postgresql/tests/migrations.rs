use howitt_postgresql::{
    Migration, MigrationError, PostgresPool, baseline_migrations, bundled_migrations,
    run_migrations,
};
use tokio_postgres::types::Type;

static CONCURRENT_MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "create_test_events",
        sql: "CREATE TABLE migration_test_events (value INTEGER NOT NULL); SELECT pg_sleep(0.1);",
    },
    Migration {
        version: 2,
        name: "insert_test_event",
        sql: "INSERT INTO migration_test_events (value) VALUES (42);",
    },
];

static FAILING_MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "create_rolled_back_table",
        sql: "CREATE TABLE migration_rolled_back (id INTEGER);",
    },
    Migration {
        version: 2,
        name: "fail_after_ddl",
        sql: "SELECT definitely_missing_function();",
    },
];

static BASELINE_MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "existing_schema",
        sql: "CREATE TABLE migration_existing (id INTEGER);",
    },
    Migration {
        version: 2,
        name: "new_schema",
        sql: "CREATE TABLE migration_new (id INTEGER);",
    },
];

async fn reset(pool: &PostgresPool) {
    pool.acquire()
        .await
        .unwrap()
        .batch_execute(
            "DROP TABLE IF EXISTS howitt_schema_migrations, migration_test_events, migration_rolled_back, migration_existing, migration_new CASCADE",
        )
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires HOWITT_MIGRATION_TEST_DATABASE_URL pointing at disposable PostgreSQL"]
async fn migration_safety_contract() {
    let url = std::env::var("HOWITT_MIGRATION_TEST_DATABASE_URL").unwrap();
    let pool = PostgresPool::connect(&url).await.unwrap();

    reset(&pool).await;
    let (first, second) = tokio::join!(
        run_migrations(&pool, CONCURRENT_MIGRATIONS),
        run_migrations(&pool, CONCURRENT_MIGRATIONS)
    );
    let mut results = [first.unwrap(), second.unwrap()];
    results.sort_by_key(|result| result.applied_versions.len());
    assert_eq!(results[0].applied_versions, Vec::<i64>::new());
    assert_eq!(results[1].applied_versions, vec![1, 2]);

    let replay = run_migrations(&pool, CONCURRENT_MIGRATIONS).await.unwrap();
    assert!(replay.applied_versions.is_empty());
    let connection = pool.acquire().await.unwrap();
    let row = connection
        .query_typed_one(
            "SELECT (SELECT count(*) FROM migration_test_events), (SELECT count(*) FROM howitt_schema_migrations)",
            &[],
        )
        .await
        .unwrap();
    assert_eq!(row.get::<_, i64>(0), 1);
    assert_eq!(row.get::<_, i64>(1), 2);

    connection
        .execute_typed(
            "UPDATE howitt_schema_migrations SET checksum = $1 WHERE version = 1",
            &[(&"0".repeat(64), Type::TEXT)],
        )
        .await
        .unwrap();
    assert!(matches!(
        run_migrations(&pool, CONCURRENT_MIGRATIONS).await,
        Err(MigrationError::ChecksumDrift { version: 1 })
    ));

    reset(&pool).await;
    assert!(run_migrations(&pool, FAILING_MIGRATIONS).await.is_err());
    let row = pool
        .acquire()
        .await
        .unwrap()
        .query_typed_one(
            "SELECT to_regclass('migration_rolled_back') IS NULL, to_regclass('howitt_schema_migrations') IS NULL",
            &[],
        )
        .await
        .unwrap();
    assert!(row.get::<_, bool>(0), "migration DDL was not rolled back");
    assert!(
        row.get::<_, bool>(1),
        "history creation was not rolled back"
    );

    reset(&pool).await;
    pool.acquire()
        .await
        .unwrap()
        .batch_execute("CREATE TABLE migration_existing (id INTEGER)")
        .await
        .unwrap();
    let baseline = baseline_migrations(&pool, BASELINE_MIGRATIONS, 1)
        .await
        .unwrap();
    assert_eq!(baseline.recorded_versions, vec![1]);
    let applied = run_migrations(&pool, BASELINE_MIGRATIONS).await.unwrap();
    assert_eq!(applied.applied_versions, vec![2]);
    let rows = pool
        .acquire()
        .await
        .unwrap()
        .query_typed(
            "SELECT version, execution_mode FROM howitt_schema_migrations ORDER BY version",
            &[],
        )
        .await
        .unwrap();
    assert_eq!(rows[0].get::<_, i64>(0), 1);
    assert_eq!(rows[0].get::<_, &str>(1), "baseline");
    assert_eq!(rows[1].get::<_, i64>(0), 2);
    assert_eq!(rows[1].get::<_, &str>(1), "applied");

    reset(&pool).await;
}

#[tokio::test]
#[ignore = "requires HOWITT_MIGRATION_TEST_DATABASE_URL pointing at disposable PostgreSQL with PostGIS"]
async fn bundled_migrations_apply_in_order_and_replay_safely() {
    let url = std::env::var("HOWITT_MIGRATION_TEST_DATABASE_URL").unwrap();
    let pool = PostgresPool::connect(&url).await.unwrap();

    let expected = bundled_migrations()
        .iter()
        .map(|migration| migration.version)
        .collect::<Vec<_>>();
    let result = run_migrations(&pool, bundled_migrations()).await.unwrap();
    assert_eq!(result.applied_versions, expected);
    assert!(
        run_migrations(&pool, bundled_migrations())
            .await
            .unwrap()
            .applied_versions
            .is_empty()
    );

    let row = pool
        .acquire()
        .await
        .unwrap()
        .query_typed_one(
            "SELECT to_regclass('routes') IS NOT NULL, to_regclass('water_beta') IS NULL, to_regclass('osm_features') IS NULL",
            &[],
        )
        .await
        .unwrap();
    assert!(row.get::<_, bool>(0));
    assert!(row.get::<_, bool>(1));
    assert!(row.get::<_, bool>(2));
}
