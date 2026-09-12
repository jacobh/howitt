use axum::{
    routing::{get, post},
    Router,
};
use http::{header, Method};
use tower_http::cors::{Any, CorsLayer};

mod app_state;
mod cache;
mod extractors;
mod graphql;
mod handlers;
mod timezone;

fn disabled_routes() -> Router {
    Router::new()
        .route("/auth/rwgps/callback", get(handlers::disabled::handler))
        .route("/upload/media", post(handlers::disabled::handler))
        .route("/webhooks/rwgps", post(handlers::disabled::handler))
        .layer(cors())
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
}

fn router(state: app_state::AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(handlers::graphql::graphiql_handler).post(handlers::graphql::graphql_handler),
        )
        .route("/auth/login", post(handlers::auth::login_handler))
        .route("/auth/signup", post(handlers::auth::signup_handler))
        .with_state(state)
        .merge(disabled_routes())
        .layer(cors())
}

#[cfg(target_arch = "wasm32")]
mod runtime {
    use super::*;
    use crate::graphql::{
        context::SchemaData,
        loaders::{
            ride_loader::RideLoader, route_points_loader::RoutePointsLoader,
            user_loader::UserLoader,
        },
        schema::build_schema,
    };
    use async_graphql::dataloader::DataLoader;
    use howitt::{
        repos::Repos,
        services::{
            fetchers::{SimplifiedRidePointsFetcher, SimplifiedTripElevationPointsFetcher},
            user::{auth::UserAuthService, signup::UserSignupService},
        },
    };
    use howitt_postgresql::{PostgresClient, PostgresRepos};
    use tower::Service;
    use worker::{postgres_tls::PassthroughTls, *};

    async fn state(env: &Env) -> anyhow::Result<app_state::AppState> {
        // No insecure fallback: missing secrets must fail closed.
        let jwt_secret = env.secret("JWT_SECRET")?.to_string();
        let hyperdrive = env.hyperdrive("HYPERDRIVE")?;
        let socket = Socket::builder()
            .secure_transport(SecureTransport::StartTls)
            .connect(hyperdrive.host(), hyperdrive.port())?;
        let config = hyperdrive
            .connection_string()
            .parse::<tokio_postgres::Config>()?;
        let (client, connection) = config.connect_raw(socket, PassthroughTls).await?;
        wasm_bindgen_futures::spawn_local(async move {
            if connection.await.is_err() {
                console_error!("PostgreSQL connection closed");
            }
        });
        let repos = Repos::from(PostgresRepos::new(PostgresClient::new(client)));
        let user_auth_service = UserAuthService::new(repos.user_repo.clone(), jwt_secret);
        let user_signup_service = UserSignupService::new(repos.user_repo.clone());
        let schema = build_schema(SchemaData {
            ride_loader: DataLoader::new(
                RideLoader::new(repos.ride_repo.clone()),
                wasm_bindgen_futures::spawn_local,
            ),
            user_loader: DataLoader::new(
                UserLoader::new(repos.user_repo.clone()),
                wasm_bindgen_futures::spawn_local,
            ),
            route_points_loader: DataLoader::new(
                RoutePointsLoader::new(repos.route_points_repo.clone()),
                wasm_bindgen_futures::spawn_local,
            ),
            simplified_ride_points_fetcher: SimplifiedRidePointsFetcher::new(
                repos.ride_points_repo.clone(),
                cache::Uncached,
            ),
            simplified_trip_elevation_points_fetcher: SimplifiedTripElevationPointsFetcher::new(
                repos.ride_repo.clone(),
                repos.ride_points_repo.clone(),
                cache::Uncached,
            ),
            repos,
            tz_finder: timezone::TimezoneLookup::new(env.get_binding("ASSETS")?),
        });
        Ok(app_state::AppState {
            schema,
            user_auth_service,
            user_signup_service,
        })
    }

    #[event(fetch)]
    async fn fetch(
        req: HttpRequest,
        env: Env,
        _ctx: worker::Context,
    ) -> worker::Result<http::Response<axum::body::Body>> {
        // These routes cannot need a database or any secrets in this first pass.
        if matches!(
            (req.method().as_str(), req.uri().path()),
            ("POST", "/upload/media" | "/webhooks/rwgps") | ("GET", "/auth/rwgps/callback")
        ) {
            return Ok(disabled_routes().call(req).await?);
        }
        let state = state(&env)
            .await
            .map_err(|_| worker::Error::RustError("Application initialization failed".into()))?;
        Ok(router(state).call(req).await?)
    }
}
