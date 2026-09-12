use chrono::Utc;
use howitt::{
    models::{
        filters::TemporalFilter,
        media::{Media, MediaFilter, MediaId, MediaRelationId},
        point::{ElevationPoint, TemporalElevationPoint},
        point_of_interest::{PointOfInterest, PointOfInterestId, PointOfInterestType},
        ride::{Ride, RideFilter, RideId, RidePoints},
        route::{Route, RouteFilter, RouteId, RoutePoints, RoutePointsFilter},
        route_description::{
            BikeSpec, DifficultyRating, Direction, Distance, RouteDescription, Scouted,
        },
        trip::{Trip, TripFilter, TripId},
        user::{User, UserFilter, UserId, UserRwgpsConnection},
    },
    repos::Repo,
    services::user::password::hash_password,
};
use howitt_postgresql::{PostgresClient, PostgresRepos};
use uuid::Uuid;

#[tokio::test]
#[ignore = "requires the disposable database created by scripts/test-worker-local.sh"]
async fn repository_codecs_and_transactions() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("HOWITT_TEST_DATABASE_URL")?;
    let config: tokio_postgres::Config = url.parse()?;
    let name = config.get_dbname().ok_or("missing test database")?;
    assert!(name.starts_with("howitt_workers_test_"));
    assert!(
        matches!(config.get_hosts(), [tokio_postgres::config::Host::Tcp(host)] if host == "127.0.0.1")
    );
    let client = PostgresClient::connect(&url).await?;
    let identity = client
        .acquire()
        .await?
        .query_typed_one("select current_database(), inet_server_addr()::text", &[])
        .await?;
    assert_eq!(identity.get::<_, String>(0), name);
    assert_eq!(identity.get::<_, String>(1), "127.0.0.1/32");
    let repos = PostgresRepos::new(client.clone());
    let user_id = UserId::from(Uuid::from_u128(1));
    let now = Utc::now();
    repos
        .user_repo
        .put(User {
            id: user_id,
            username: "worker-test".into(),
            email: "worker-test@example.invalid".into(),
            password: hash_password("local-test-password").unwrap(),
            created_at: now,
            rwgps_connection: None,
        })
        .await?;
    let mut user = repos.user_repo.get(user_id).await?;
    assert!(user.rwgps_connection.is_none());
    for filter in [
        UserFilter::Username(user.username.clone()),
        UserFilter::Email(user.email.clone()),
        UserFilter::Ids(vec![user_id]),
    ] {
        assert_eq!(repos.user_repo.filter_models(filter).await?.len(), 1);
    }
    user.rwgps_connection = Some(UserRwgpsConnection {
        id: Uuid::from_u128(10),
        user_id,
        rwgps_user_id: 123,
        access_token: "synthetic-token".into(),
        created_at: now,
        updated_at: now,
    });
    repos.user_repo.put(user).await?;
    assert_eq!(
        repos
            .user_repo
            .filter_models(UserFilter::RwgpsId(123))
            .await?
            .len(),
        1
    );
    assert_eq!(repos.user_repo.all().await?.len(), 1);

    let route_id = RouteId::from(Uuid::from_u128(2));
    let elevation_point = ElevationPoint {
        point: geo::Point::new(144.96, -37.81),
        elevation: 20.0,
    };
    repos
        .route_repo
        .put(Route {
            id: route_id,
            user_id,
            name: "Test route".into(),
            slug: "test-route".into(),
            distance: 100.0,
            sample_points: Some(vec![elevation_point.clone()]),
            description: Some(RouteDescription {
                description: Some("Synthetic description".into()),
                published_at: Some(now),
                technical_difficulty: Some(DifficultyRating::Blue),
                physical_difficulty: Some(DifficultyRating::Black),
                minimum_bike: Some(BikeSpec {
                    tyre_width: Distance::Millimeters(50.0).into(),
                    front_suspension: Default::default(),
                    rear_suspension: Default::default(),
                }),
                ideal_bike: None,
                scouted: Some(Scouted::Yes),
                direction: Some(Direction::Either),
                tags: vec!["synthetic".into()],
            }),
            external_ref: None,
            tags: Default::default(),
        })
        .await?;
    repos
        .route_points_repo
        .put(RoutePoints {
            id: route_id,
            points: vec![elevation_point],
        })
        .await?;
    assert_eq!(
        repos
            .route_repo
            .get(route_id)
            .await?
            .sample_points
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        repos
            .route_repo
            .filter_models(RouteFilter::UserId(user_id))
            .await?
            .len(),
        1
    );
    assert_eq!(repos.route_points_repo.get(route_id).await?.points.len(), 1);
    assert_eq!(repos.route_points_repo.all().await?.len(), 1);
    assert_eq!(
        repos
            .route_points_repo
            .filter_models(RoutePointsFilter::Ids(vec![route_id]))
            .await?
            .len(),
        1
    );
    for filter in [RouteFilter::All, RouteFilter::Slug("test-route".into())] {
        assert_eq!(repos.route_repo.filter_models(filter).await?.len(), 1);
    }
    assert!(repos
        .route_repo
        .filter_models(RouteFilter::RwgpsId(123))
        .await?
        .is_empty());
    assert!(repos
        .route_repo
        .filter_models(RouteFilter::Starred)
        .await?
        .is_empty());
    let description = repos.route_repo.all().await?.remove(0).description.unwrap();
    assert_eq!(
        description.technical_difficulty,
        Some(DifficultyRating::Blue)
    );
    assert_eq!(description.tags, vec!["synthetic"]);
    assert!(description.minimum_bike.is_some());

    let ride_id = RideId::from(Uuid::from_u128(3));
    repos
        .ride_repo
        .put(Ride {
            id: ride_id,
            user_id,
            name: "Test ride".into(),
            distance: 100.0,
            started_at: now,
            finished_at: now,
            external_ref: None,
        })
        .await?;
    repos
        .ride_points_repo
        .put(RidePoints {
            id: ride_id,
            points: vec![TemporalElevationPoint {
                point: geo::Point::new(144.96, -37.81),
                elevation: 20.0,
                datetime: now,
            }],
        })
        .await?;
    assert_eq!(repos.ride_repo.get(ride_id).await?.name, "Test ride");
    assert_eq!(repos.ride_points_repo.get(ride_id).await?.points.len(), 1);
    assert_eq!(
        repos
            .ride_repo
            .filter_models(RideFilter::Ids(vec![ride_id]))
            .await?
            .len(),
        1
    );
    assert_eq!(
        repos
            .ride_repo
            .filter_models(RideFilter::ForUser {
                user_id,
                started_at: Some(TemporalFilter::After {
                    after: now - chrono::Duration::days(1),
                    first: Some(10)
                })
            })
            .await?
            .len(),
        1
    );

    let poi_id = PointOfInterestId::from(Uuid::from_u128(4));
    repos
        .point_of_interest_repo
        .put(PointOfInterest {
            id: poi_id,
            user_id,
            name: "Test water".into(),
            slug: "test-water".into(),
            point: geo::Point::new(144.96, -37.81),
            point_of_interest_type: PointOfInterestType::WaterSource,
            description: Some("Synthetic".into()),
        })
        .await?;
    assert_eq!(
        repos.point_of_interest_repo.get(poi_id).await?.point.x(),
        144.96
    );

    let trip_id = TripId::from(Uuid::from_u128(5));
    let trip = Trip {
        id: trip_id,
        user_id,
        name: "Test trip".into(),
        slug: "test-trip".into(),
        year: 2026,
        created_at: now,
        description: None,
        notes: Default::default(),
        ride_ids: vec![ride_id],
        media_ids: vec![],
        is_published: true,
    };
    repos.trip_repo.put(trip.clone()).await?;
    assert_eq!(repos.trip_repo.get(trip_id).await?.ride_ids, vec![ride_id]);
    assert_eq!(
        repos.trip_repo.filter_models(TripFilter::All).await?.len(),
        1
    );
    let media_id = MediaId::from(Uuid::from_u128(6));
    let media = Media {
        id: media_id,
        user_id,
        created_at: now,
        path: "synthetic.jpg".into(),
        point: Some(geo::Point::new(144.96, -37.81)),
        captured_at: Some(now),
        relation_ids: vec![
            MediaRelationId::from(ride_id),
            MediaRelationId::from(route_id),
            MediaRelationId::from(trip_id),
            MediaRelationId::from(poi_id),
        ],
    };
    repos.media_repo.put(media.clone()).await?;
    assert_eq!(repos.media_repo.get(media_id).await?.relation_ids.len(), 4);
    assert_eq!(repos.media_repo.all().await?.len(), 1);

    // Force a foreign-key failure after the initial UPDATE and relation DELETE;
    // dropping the transaction must roll everything back before the next query.
    let mut invalid_trip = trip;
    invalid_trip.name = "Must roll back".into();
    invalid_trip.ride_ids = vec![RideId::from(Uuid::from_u128(999))];
    assert!(repos.trip_repo.put(invalid_trip).await.is_err());
    assert_eq!(repos.trip_repo.get(trip_id).await?.name, "Test trip");
    assert_eq!(repos.trip_repo.get(trip_id).await?.ride_ids, vec![ride_id]);
    let mut invalid_media = media;
    invalid_media.path = "must-rollback.jpg".into();
    invalid_media.relation_ids = vec![MediaRelationId::from(RideId::from(Uuid::from_u128(999)))];
    assert!(repos.media_repo.put(invalid_media).await.is_err());
    assert_eq!(repos.media_repo.get(media_id).await?.path, "synthetic.jpg");
    assert_eq!(repos.media_repo.get(media_id).await?.relation_ids.len(), 4);
    repos
        .media_repo
        .put(Media {
            id: MediaId::from(Uuid::from_u128(7)),
            user_id,
            created_at: now,
            path: "boundary.jpg".into(),
            point: Some(geo::Point::new(141.0, -33.0)),
            captured_at: None,
            relation_ids: vec![MediaRelationId::from(trip_id)],
        })
        .await?;
    // Exercise the remaining typed parameter shapes on the same connection.
    for filter in [
        RideFilter::All,
        RideFilter::ForUser {
            user_id,
            started_at: None,
        },
        RideFilter::ForUser {
            user_id,
            started_at: Some(TemporalFilter::Before {
                before: now + chrono::Duration::days(1),
                last: Some(10),
            }),
        },
        RideFilter::ForUserWithDate {
            user_id,
            date: now
                .with_timezone(&chrono_tz::Australia::Melbourne)
                .date_naive(),
        },
        RideFilter::ForTrip(trip_id),
    ] {
        assert_eq!(repos.ride_repo.filter_models(filter).await?.len(), 1);
    }
    assert!(repos
        .ride_repo
        .filter_models(RideFilter::RwgpsId(123))
        .await?
        .is_empty());
    assert_eq!(repos.ride_repo.all().await?.len(), 1);
    assert_eq!(repos.ride_points_repo.all().await?.len(), 1);
    assert_eq!(repos.point_of_interest_repo.all().await?.len(), 1);
    assert_eq!(
        repos.point_of_interest_repo.filter_models(()).await?.len(),
        1
    );
    for filter in [
        TripFilter::User(user_id),
        TripFilter::WithUserAndSlug {
            user_id,
            slug: "test-trip".into(),
        },
        TripFilter::Published,
    ] {
        assert_eq!(repos.trip_repo.filter_models(filter).await?.len(), 1);
    }
    for filter in [
        MediaFilter::Ids(vec![media_id]),
        MediaFilter::ForRide(ride_id),
        MediaFilter::ForRoute(route_id),
        MediaFilter::ForPointOfInterest(poi_id),
    ] {
        assert_eq!(repos.media_repo.filter_models(filter).await?.len(), 1);
    }
    for filter in [MediaFilter::ForUser(user_id), MediaFilter::ForTrip(trip_id)] {
        assert_eq!(repos.media_repo.filter_models(filter).await?.len(), 2);
    }
    let connection = client.acquire().await?;
    let prepared_count: i64 = connection
        .query_typed_one("select count(*) from pg_prepared_statements", &[])
        .await?
        .get(0);
    assert_eq!(
        prepared_count, 0,
        "repository operations must not leave named statements"
    );
    let backend: i32 = connection
        .query_typed_one("select pg_backend_pid()", &[])
        .await?
        .get(0);
    assert!(connection
        .query_typed_one("select pg_terminate_backend(pg_backend_pid())", &[])
        .await
        .is_err());
    // A server's fatal response arrives before the driver closes its channel.
    // Observe actual closure before testing acquisition of a replacement.
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while !connection.is_closed() {
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    })
    .await?;
    drop(connection);
    let replacement: i32 = client
        .acquire()
        .await?
        .query_typed_one("select pg_backend_pid()", &[])
        .await?
        .get(0);
    assert_ne!(backend, replacement);
    assert_eq!(repos.user_repo.get(user_id).await?.username, "worker-test");
    Ok(())
}
