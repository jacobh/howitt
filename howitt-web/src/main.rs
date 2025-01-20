#![feature(async_closure)]
use std::net::SocketAddr;
use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use auth::login_handler;
use axum::{
    extract::{FromRef, FromRequestParts, State},
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
use http::{header, request::Parts, StatusCode};
use slog::Drain;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};

mod auth;
mod graphql;

use graphql::{
    context::{RequestData, SchemaData},
    credentials::Credentials,
    Query,
};

#[derive(Clone)]
struct AppState {
    pub schema: Schema<Query, EmptyMutation, EmptySubscription>,
    pub user_auth_service: UserAuthService,
}

impl FromRef<AppState> for Schema<Query, EmptyMutation, EmptySubscription> {
    fn from_ref(state: &AppState) -> Self {
        state.schema.clone()
    }
}

impl FromRef<AppState> for UserAuthService {
    fn from_ref(state: &AppState) -> Self {
        state.user_auth_service.clone()
    }
}

fn new_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, slog::o!())
}

// Custom extractor for auth
struct OptionalLogin(Option<Login>);

#[async_trait::async_trait]
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
    State(schema): State<Schema<Query, EmptyMutation, EmptySubscription>>,
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

    let logger = new_logger();

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

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(SchemaData {
            poi_repo,
            route_repo,
            ride_repo,
            user_repo: user_repo.clone(),
            simplified_ride_points_fetcher,
        })
        .finish();

    let app_state = AppState {
        schema,
        user_auth_service,
    };

    println!("GraphiQL IDE: http://localhost:8000");

    // // Build our middleware stack
    // let cors = CorsLayer::new()
    //     .allow_origin(Any)
    //     .allow_methods(Any)
    //     .allow_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE]);
    // // .allow_headers(
    // //     vec![AUTHORIZATION, axum::http::header::CONTENT_TYPE]
    // //         .into_iter()
    // //         .collect::<Vec<_>>(),
    // // );

    // let auth_layer = middleware::from_fn(
    //     move |req: axum::http::Request<axum::body::Body>, next: Next<axum::body::Body>| {
    //         let logger = logger_clone.clone();
    //         async move {
    //             let start = std::time::Instant::now();
    //             let method = req.method().clone();
    //             let uri = req.uri().clone();

    //             let response = next.run(req).await;

    //             let duration_ms = start.elapsed().as_millis();
    //             let status = response.status().as_u16();

    //             slog::info!(logger, "{} {}", method, uri;
    //                 "status" => status,
    //                 "ms" => duration_ms
    //             );

    //             Ok(response)
    //         }
    //     },
    // );

    // Create the router
    let app = Router::new()
        .route("/", get(graphiql_handler).post(graphql_handler))
        .route("/auth/login", post(login_handler))
        .with_state(app_state);
    // .layer(cors)
    // .layer(CompressionLayer::new())
    // .layer(auth_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
