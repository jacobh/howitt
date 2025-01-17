#![feature(async_closure)]
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use howitt::services::{fetchers::SimplifiedRidePointsFetcher, user::auth::UserAuthService};
use howitt_clients::RedisClient;
use howitt_postgresql::{
    PostgresClient, PostgresPointOfInterestRepo, PostgresRidePointsRepo, PostgresRideRepo,
    PostgresRouteRepo, PostgresUserRepo,
};
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

use graphql::{context::SchemaData, schema::build_schema};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
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

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "asdf123".to_string());

    let poi_repo = Arc::new(PostgresPointOfInterestRepo::new(pg.clone()));
    let route_repo = Arc::new(PostgresRouteRepo::new(pg.clone()));
    let ride_repo = Arc::new(PostgresRideRepo::new(pg.clone()));
    let ride_points_repo = Arc::new(PostgresRidePointsRepo::new(pg.clone()));
    let user_repo = Arc::new(PostgresUserRepo::new(pg.clone()));

    let user_auth_service = UserAuthService::new(user_repo.clone(), jwt_secret);
    let simplified_ride_points_fetcher = SimplifiedRidePointsFetcher {
        ride_points_repo: ride_points_repo.clone(),
        redis_client: redis,
    };

    let schema = build_schema(SchemaData {
        poi_repo,
        route_repo,
        ride_repo,
        user_repo,
        simplified_ride_points_fetcher,
    });

    let app_state = app_state::AppState {
        schema,
        user_auth_service,
    };

    let app = Router::new()
        .route(
            "/",
            get(handlers::graphql::graphiql_handler).post(handlers::graphql::graphql_handler),
        )
        .route("/auth/login", post(handlers::auth::login_handler))
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

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
