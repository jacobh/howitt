#![feature(async_closure)]
use std::net::SocketAddr;
use std::sync::Arc;

use apalis_redis::RedisStorage;
use async_graphql::dataloader::DataLoader;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use howitt::{
    jobs::Job,
    repos::Repos,
    services::{fetchers::SimplifiedRidePointsFetcher, user::auth::UserAuthService},
};
use howitt_client_types::BucketName;
use howitt_clients::{RedisClient, S3BucketClient};
use howitt_postgresql::{PostgresClient, PostgresRepos};
use http::{header, Method};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

mod app_state;
mod extractors;
mod graphql;
mod handlers;

use graphql::{
    context::SchemaData,
    loaders::{route_points_loader::RoutePointsLoader, user_loader::UserLoader},
    schema::build_schema,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pg = PostgresClient::connect(
        &std::env::var("DATABASE_URL")
            .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
    )
    .await?;

    let redis = RedisClient::connect(
        std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1/")),
    )
    .await?;

    let conn = apalis_redis::connect(
        std::env::var("REDIS_URL").unwrap_or(String::from("redis://127.0.0.1:6379/")),
    )
    .await?;

    let job_storage: RedisStorage<Job> = RedisStorage::new(conn);

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "asdf123".to_string());

    let postgres_repos = PostgresRepos::new(pg);
    let repos: Repos = postgres_repos.clone().into();

    let user_auth_service = UserAuthService::new(repos.user_repo.clone(), jwt_secret);
    let simplified_ride_points_fetcher =
        SimplifiedRidePointsFetcher::new(repos.ride_points_repo.clone(), redis);

    let bucket_client = S3BucketClient::new_from_env(BucketName::Media);

    let schema = build_schema(SchemaData {
        poi_repo: repos.point_of_interest_repo.clone(),
        route_repo: repos.route_repo.clone(),
        ride_repo: repos.ride_repo.clone(),
        user_repo: repos.user_repo.clone(),
        trip_repo: repos.trip_repo.clone(),
        media_repo: repos.media_repo.clone(),
        user_loader: DataLoader::new(UserLoader::new(repos.user_repo.clone()), tokio::spawn),
        route_points_loader: DataLoader::new(
            RoutePointsLoader::new(repos.route_points_repo.clone()),
            tokio::spawn,
        ),
        simplified_ride_points_fetcher,
        rwgps_client_id: std::env::var("RWGPS_CLIENT_ID").expect("RWGPS_CLIENT_ID must be set"),
        user_auth_service: user_auth_service.clone(),
    });

    let app_state = app_state::AppState {
        schema,
        user_auth_service,
        media_repo: Arc::new(postgres_repos.media_repo.clone()),
        bucket_client: Arc::new(bucket_client),
        job_storage: Arc::new(tokio::sync::Mutex::new(job_storage)),
        user_repo: Arc::new(postgres_repos.user_repo.clone()),
        rwgps: app_state::RwgpsConfig {
            client_id: std::env::var("RWGPS_CLIENT_ID").expect("RWGPS_CLIENT_ID must be set"),
            client_secret: std::env::var("RWGPS_CLIENT_SECRET")
                .expect("RWGPS_CLIENT_SECRET must be set"),
            redirect_uri: std::env::var("RWGPS_REDIRECT_URI")
                .expect("RWGPS_REDIRECT_URI must be set"),
        },
    };
    let app = Router::new()
        .route(
            "/",
            get(handlers::graphql::graphiql_handler).post(handlers::graphql::graphql_handler),
        )
        .route("/auth/login", post(handlers::auth::login_handler))
        .route(
            "/auth/rwgps/callback",
            get(handlers::rwgps::rwgps_callback_handler),
        )
        .route(
            "/upload/media",
            post(handlers::upload::upload_media_handler)
                .layer(DefaultBodyLimit::max(1024 * 1024 * 100)),
        )
        .route(
            "/webhooks/rwgps",
            post(handlers::rwgps::rwgps_webhook_handler),
        )
        .with_state(app_state)
        .layer(
            tower::ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &axum::http::Request<_>| {
                            tracing::info_span!(
                                "http_request",
                                path = %request.uri().path(),
                                method = %request.method(),
                            )
                        })
                        .on_response(
                            |resp: &http::Response<_>,
                             latency: std::time::Duration,
                             _span: &tracing::Span| {
                                tracing::info!(
                                    latency = %format!("{}ms", latency.as_millis()),
                                    status = %resp.status(),
                                );
                            },
                        ),
                )
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST])
                        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]),
                ),
        );

    let addr = std::env::var("BIND_ADDRESS")
        .unwrap_or_default()
        .parse::<SocketAddr>()
        .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], 8000)));

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
