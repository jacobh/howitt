use chrono::{Duration, Utc};
use howitt::{
    models::{
        external_ref::{ExternalId, ExternalRef, RwgpsId},
        point::{ElevationPoint, TemporalElevationPoint},
        ride::{Ride, RideId},
        route::{Route, RouteId},
        user::UserId,
    },
    services::sync::rwgps_v2::persistence::RwgpsSyncStore,
};
use howitt_postgresql::{PostgresPool, PostgresRwgpsSyncStore};
use tokio_postgres::types::Type;
use uuid::Uuid;

fn route(
    id: u128,
    user_id: UserId,
    external_id: usize,
    updated_at: chrono::DateTime<Utc>,
) -> Route {
    Route {
        id: RouteId::from(Uuid::from_u128(id)),
        name: format!("route-{id}"),
        slug: format!("route-{id}"),
        user_id,
        distance: id as f64,
        sample_points: Some(vec![]),
        description: None,
        external_ref: Some(ExternalRef {
            id: ExternalId::Rwgps(RwgpsId::Route(external_id)),
            updated_at,
            sync_version: Some(2),
        }),
        tags: Default::default(),
    }
}

async fn disposable_pool() -> Result<PostgresPool, Box<dyn std::error::Error>> {
    let url = std::env::var("HOWITT_TEST_DATABASE_URL")?;
    let config: tokio_postgres::Config = url.parse()?;
    let name = config.get_dbname().ok_or("missing test database")?;
    assert!(name.starts_with("howitt_workers_test_"));
    assert!(matches!(
        config.get_hosts(),
        [tokio_postgres::config::Host::Tcp(host)] if host == "127.0.0.1"
    ));
    Ok(PostgresPool::connect(&url).await?)
}

#[tokio::test]
#[ignore = "requires the disposable database created by scripts/test-worker-local.sh"]
async fn route_deliveries_are_atomic_concurrent_and_stale_safe()
-> Result<(), Box<dyn std::error::Error>> {
    let pool = disposable_pool().await?;
    let conn = pool.acquire().await?;
    let user_uuid = Uuid::from_u128(0x5257_4750_5300_0000_0000_0000_0000_0001);
    conn.execute_typed(
        "insert into users (id, username, password, email) values ($1, $2, 'x', $3)",
        &[
            (&user_uuid, Type::UUID),
            (&format!("rwgps-{user_uuid}"), Type::VARCHAR),
            (&format!("rwgps-{user_uuid}@example.invalid"), Type::VARCHAR),
        ],
    )
    .await?;
    let user_id = UserId::from(user_uuid);
    let now = Utc::now();
    let first = route(10, user_id, 987_654, now);
    let duplicate = route(11, user_id, 987_654, now);
    let a = PostgresRwgpsSyncStore::new(pool.clone());
    let b = a.clone();
    let (left, right) = tokio::join!(a.save_route(first, vec![]), b.save_route(duplicate, vec![]));
    left.unwrap();
    right.unwrap();

    let row = conn
        .query_typed_one(
            "select id, distance_m from routes where (external_ref->'id'->'Rwgps'->'Route')::int = 987654",
            &[],
        )
        .await?;
    let persisted_id: Uuid = row.get(0);
    let persisted_distance: i32 = row.get(1);
    assert!([10, 11].contains(&persisted_distance));
    let point = ElevationPoint {
        point: geo::Point::new(115.8, -31.9),
        elevation: 47.0,
    };
    // Equal timestamps must repair missing points from the old non-atomic writer.
    conn.execute_typed(
        "delete from route_points where route_id=$1",
        &[(&persisted_id, Type::UUID)],
    )
    .await?;
    a.save_route(route(12, user_id, 987_654, now), vec![point.clone()])
        .await
        .unwrap();
    let persisted_distance = 12;
    let persisted_points: serde_json::Value = conn
        .query_typed_one(
            "select points from route_points where route_id=$1",
            &[(&persisted_id, Type::UUID)],
        )
        .await?
        .get(0);
    assert_eq!(persisted_points, serde_json::json!([[115.8, -31.9, 47.0]]));
    assert!(
        a.save_route(
            route(
                14,
                UserId::from(Uuid::from_u128(999)),
                987_654,
                now + Duration::seconds(2)
            ),
            vec![]
        )
        .await
        .is_err()
    );
    a.save_route(
        route(12, user_id, 987_654, now - Duration::seconds(1)),
        vec![],
    )
    .await
    .unwrap();
    assert_eq!(
        conn.query_typed_one(
            "select distance_m from routes where id=$1",
            &[(&persisted_id, Type::UUID)]
        )
        .await?
        .get::<_, i32>(0),
        persisted_distance
    );

    conn.execute_typed(
        "create function fail_rwgps_points() returns trigger language plpgsql as $$ begin raise exception 'forced point failure'; end $$",
        &[],
    ).await?;
    conn.execute_typed(
        "create trigger fail_rwgps_points before insert or update on route_points for each row execute function fail_rwgps_points()",
        &[],
    ).await?;
    let failed = a
        .save_route(
            route(13, user_id, 987_654, now + Duration::seconds(1)),
            vec![ElevationPoint {
                point: geo::Point::new(1.0, 2.0),
                elevation: 3.0,
            }],
        )
        .await;
    assert!(failed.is_err());
    assert_eq!(
        conn.query_typed_one(
            "select distance_m from routes where id=$1",
            &[(&persisted_id, Type::UUID)]
        )
        .await?
        .get::<_, i32>(0),
        persisted_distance
    );
    assert!(
        a.save_route(route(15, user_id, 987_655, now), vec![point])
            .await
            .is_err()
    );
    assert!(
        conn.query_typed_opt(
            "select id from routes where (external_ref->'id'->'Rwgps'->'Route')::int=987655",
            &[]
        )
        .await?
        .is_none()
    );
    assert_eq!(
        conn.query_typed_one(
            "select points from route_points where route_id=$1",
            &[(&persisted_id, Type::UUID)]
        )
        .await?
        .get::<_, serde_json::Value>(0),
        persisted_points
    );
    conn.execute_typed("drop trigger fail_rwgps_points on route_points", &[])
        .await?;
    conn.execute_typed("drop function fail_rwgps_points()", &[])
        .await?;
    // Replay the failed update, retaining user-owned metadata and the original ID.
    conn.execute_typed(
        "update routes set name='edited', description='keep me', tags=ARRAY['custom'] where id=$1",
        &[(&persisted_id, Type::UUID)],
    )
    .await?;
    a.save_route(
        route(16, user_id, 987_654, now + Duration::seconds(1)),
        vec![],
    )
    .await
    .unwrap();
    let updated = conn
        .query_typed_one(
            "select name, description, tags, distance_m from routes where id=$1",
            &[(&persisted_id, Type::UUID)],
        )
        .await?;
    assert_eq!(updated.get::<_, String>(0), "edited");
    assert_eq!(updated.get::<_, String>(1), "keep me");
    assert_eq!(updated.get::<_, Vec<String>>(2), vec!["custom"]);
    assert_eq!(updated.get::<_, i32>(3), 16);
    Ok(())
}

#[tokio::test]
#[ignore = "requires the disposable database created by scripts/test-jobs-local.sh"]
async fn trip_deliveries_preserve_identity_and_rollback_points()
-> Result<(), Box<dyn std::error::Error>> {
    let pool = disposable_pool().await?;
    let conn = pool.acquire().await?;
    let user_uuid = Uuid::from_u128(0x5257_4750_5300_0000_0000_0000_0000_0002);
    conn.execute_typed("insert into users (id, username, password, email) values ($1, 'rwgps-trip', 'x', 'rwgps-trip@example.invalid')", &[(&user_uuid, Type::UUID)]).await?;
    let now = Utc::now();
    let make_ride = |id, offset| Ride {
        id: RideId::from(Uuid::from_u128(id)),
        user_id: UserId::from(user_uuid),
        name: "imported".into(),
        distance: 1200.0,
        started_at: now + Duration::seconds(offset),
        finished_at: now + Duration::seconds(offset + 10),
        external_ref: Some(ExternalRef {
            id: ExternalId::Rwgps(RwgpsId::Trip(876_543)),
            sync_version: Some(2),
            updated_at: now + Duration::seconds(offset),
        }),
    };
    let point = TemporalElevationPoint {
        point: geo::Point::new(120.0, -35.0),
        elevation: 81.0,
        datetime: now,
    };
    let store = PostgresRwgpsSyncStore::new(pool.clone());
    let (a, b) = tokio::join!(
        store.save_trip(make_ride(100, 0), vec![point.clone()]),
        store.save_trip(make_ride(101, 0), vec![point.clone()])
    );
    a.unwrap();
    b.unwrap();
    let rows = conn
        .query_typed(
            "select id from rides where (external_ref->'id'->'Rwgps'->'Trip')::int=876543",
            &[],
        )
        .await?;
    assert_eq!(rows.len(), 1);
    let id: Uuid = rows[0].get(0);
    conn.execute_typed(
        "update rides set name='edited trip', distance_m=999 where id=$1",
        &[(&id, Type::UUID)],
    )
    .await?;
    store
        .save_trip(make_ride(102, 2), vec![point.clone()])
        .await
        .unwrap();
    store.save_trip(make_ride(103, 1), vec![]).await.unwrap();
    let saved = conn
        .query_typed_one(
            "select name, distance_m, started_at from rides where id=$1",
            &[(&id, Type::UUID)],
        )
        .await?;
    assert_eq!(saved.get::<_, String>(0), "edited trip");
    assert_eq!(saved.get::<_, i32>(1), 999);
    // PostgreSQL timestamps round to microseconds.
    assert_eq!(
        saved.get::<_, chrono::DateTime<Utc>>(2).timestamp_micros(),
        (now + Duration::seconds(2)).timestamp_micros()
    );
    let mut wrong_user = make_ride(104, 3);
    wrong_user.user_id = UserId::from(Uuid::from_u128(999));
    assert!(store.save_trip(wrong_user, vec![]).await.is_err());
    conn.execute_typed("create function fail_rwgps_trip_points() returns trigger language plpgsql as $$ begin raise exception 'forced trip point failure'; end $$", &[]).await?;
    conn.execute_typed("create trigger fail_rwgps_trip_points before insert or update on ride_points for each row execute function fail_rwgps_trip_points()", &[]).await?;
    assert!(store.save_trip(make_ride(105, 4), vec![]).await.is_err());
    let after = conn.query_typed_one("select started_at, p.points from rides r join ride_points p on p.ride_id=r.id where r.id=$1", &[(&id, Type::UUID)]).await?;
    assert_eq!(
        after.get::<_, chrono::DateTime<Utc>>(0),
        saved.get::<_, chrono::DateTime<Utc>>(2)
    );
    assert_eq!(after.get::<_, serde_json::Value>(1)[0][3], 81.0);
    conn.execute_typed("drop trigger fail_rwgps_trip_points on ride_points", &[])
        .await?;
    conn.execute_typed("drop function fail_rwgps_trip_points()", &[])
        .await?;
    store.save_trip(make_ride(105, 4), vec![]).await.unwrap();
    Ok(())
}
