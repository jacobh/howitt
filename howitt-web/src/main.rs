#![feature(async_closure)]
use std::net::SocketAddr;
use std::sync::Arc;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use auth::login_handler;
use axum::{
    extract::{FromRequestParts, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use howitt::services::{
    fetchers::SimplifiedRidePointsFetcher,
    user::auth::{Login, UserAuthService},
};
use howitt_clients::RedisClient;
use howitt_postgresql::{
    PostgresClient, PostgresPointOfInterestRepo, PostgresRidePointsRepo, PostgresRideRepo,
    PostgresRouteRepo, PostgresUserRepo,
};
use http::{header, request::Parts, Method, StatusCode};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

mod auth;
mod graphql;

use graphql::{
    context::{RequestData, SchemaData},
    credentials::Credentials,
    schema::{build_schema, Schema},
};

#[derive(axum_macros::FromRef, Clone)]
struct AppState {
    pub schema: Schema,
    pub user_auth_service: UserAuthService,
}

// Custom extractor for auth
struct OptionalLogin(Option<Login>);

impl FromRequestParts<AppState> for OptionalLogin {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let AppState {
            user_auth_service, ..
        } = state;

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let credentials = auth_header.and_then(|s| Credentials::parse_auth_header_value(s).ok());

        match credentials {
            Some(Credentials::BearerToken(token)) => match user_auth_service.verify(&token).await {
                Ok(login) => Ok(OptionalLogin(Some(login))),
                Err(_) => Ok(OptionalLogin(None)),
            },
            Some(Credentials::Key(_)) => Ok(OptionalLogin(None)),
            None => Ok(OptionalLogin(None)),
        }
    }
}

async fn graphql_handler(
    State(schema): State<Schema>,
    OptionalLogin(login): OptionalLogin,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    request = request.data(RequestData { login });
    schema.execute(request).await.into()
}

async fn graphiql_handler() -> impl IntoResponse {
    Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/")
            .finish(),
    )
}

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

    let app_state = AppState {
        schema,
        user_auth_service,
    };

    let app = Router::new()
        .route("/", get(graphiql_handler).post(graphql_handler))
        .route("/auth/login", post(login_handler))
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
