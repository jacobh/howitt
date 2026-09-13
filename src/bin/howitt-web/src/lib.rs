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
pub use graphql::observability::ResolverTracing;

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
            ride_loader::RideLoader,
            route_data_loader::RouteDataLoader,
            trip_content_loader::{TripMediaLoader, TripRidesLoader},
            user_loader::UserLoader,
        },
        schema::build_schema,
    };
    use async_graphql::dataloader::{DataLoader, HashMapCache};
    use howitt::{
        repos::Repos,
        services::{
            fetchers::{SimplifiedRidePointsFetcher, SimplifiedTripElevationPointsFetcher},
            user::{auth::UserAuthService, signup::UserSignupService},
        },
    };
    use howitt_postgresql::{PostgresPool, PostgresRepos};
    use tower::Service;
    use worker::*;

    async fn state(env: &Env) -> anyhow::Result<app_state::AppState> {
        // No insecure fallback: missing secrets must fail closed.
        let jwt_secret = env.secret("JWT_SECRET")?.to_string();
        let pool = PostgresPool::from_hyperdrive(env.hyperdrive("HYPERDRIVE")?)?;
        let repos = Repos::from(PostgresRepos::new(pool));
        let user_auth_service = UserAuthService::new(repos.user_repo.clone(), jwt_secret);
        let user_signup_service = UserSignupService::new(repos.user_repo.clone());
        let cache = cache::WorkerCache::new(env.kv("DERIVED_CACHE")?);
        let schema = build_schema(SchemaData {
            ride_loader: DataLoader::with_cache(
                RideLoader::new(repos.ride_repo.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            ),
            user_loader: DataLoader::with_cache(
                UserLoader::new(repos.user_repo.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            ),
            trip_rides_loader: DataLoader::with_cache(
                TripRidesLoader(repos.ride_repo.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            ),
            trip_media_loader: DataLoader::with_cache(
                TripMediaLoader(repos.media_repo.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            ),
            route_points_loader: DataLoader::with_cache(
                RouteDataLoader::new(repos.route_points_repo.clone(), cache.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            ),
            simplified_ride_points_fetcher: SimplifiedRidePointsFetcher::new(
                repos.ride_points_repo.clone(),
                cache.clone(),
            ),
            simplified_trip_elevation_points_fetcher: SimplifiedTripElevationPointsFetcher::new(
                repos.ride_repo.clone(),
                repos.ride_points_repo.clone(),
                cache,
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
