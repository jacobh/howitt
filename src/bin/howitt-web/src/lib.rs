use axum::{
    Router,
    routing::{get, post},
};
use http::{Method, header};
use tower_http::cors::{Any, CorsLayer};

mod app_state;
mod cache;
mod extractors;
mod graphql;
mod handlers;
mod jobs;
mod timezone;
pub use graphql::observability::ResolverTracing;

fn webhook_router(state: app_state::RwgpsWebhookState) -> Router {
    Router::new()
        .route("/webhooks/rwgps", post(handlers::rwgps::webhook_handler))
        .with_state(state)
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
        .route(
            "/auth/rwgps/callback",
            get(handlers::rwgps::auth_callback_handler),
        )
        .route(
            "/upload/media",
            post(handlers::media::handler).layer(axum::extract::DefaultBodyLimit::max(
                handlers::media::MAX_FILE_BYTES + 16_384,
            )),
        )
        .with_state(state)
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
    use std::sync::Arc;
    use tower::Service;
    use worker::*;

    // async-graphql 7.2.1's batching delay is not implemented for wasm32.
    // Loading immediately avoids its panicking timer while retaining caching.
    const DATALOADER_MAX_BATCH_SIZE: usize = 1;

    fn rwgps_config(env: &Env) -> anyhow::Result<app_state::RwgpsConfig> {
        Ok(app_state::RwgpsConfig {
            client_id: env.secret("RWGPS_CLIENT_ID")?.to_string(),
            client_secret: env.secret("RWGPS_CLIENT_SECRET")?.to_string(),
            redirect_uri: env.var("RWGPS_REDIRECT_URI")?.to_string(),
        })
    }

    fn job_queue(env: &Env) -> anyhow::Result<jobs::DynJobQueue> {
        Ok(Arc::new(jobs::CloudflareJobQueue::new(env.queue("JOBS")?)))
    }

    async fn state(env: &Env) -> anyhow::Result<app_state::AppState> {
        // No insecure fallback: missing secrets must fail closed.
        let jwt_secret = env.secret("JWT_SECRET")?.to_string();
        let rwgps = rwgps_config(env)?;
        let jobs = job_queue(env)?;
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
            )
            .max_batch_size(DATALOADER_MAX_BATCH_SIZE),
            user_loader: DataLoader::with_cache(
                UserLoader::new(repos.user_repo.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            )
            .max_batch_size(DATALOADER_MAX_BATCH_SIZE),
            trip_rides_loader: DataLoader::with_cache(
                TripRidesLoader(repos.ride_repo.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            )
            .max_batch_size(DATALOADER_MAX_BATCH_SIZE),
            trip_media_loader: DataLoader::with_cache(
                TripMediaLoader(repos.media_repo.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            )
            .max_batch_size(DATALOADER_MAX_BATCH_SIZE),
            route_points_loader: DataLoader::with_cache(
                RouteDataLoader::new(repos.route_points_repo.clone(), cache.clone()),
                howitt_observability::spawn_local,
                HashMapCache::default(),
            )
            .max_batch_size(DATALOADER_MAX_BATCH_SIZE),
            simplified_ride_points_fetcher: SimplifiedRidePointsFetcher::new(
                repos.ride_points_repo.clone(),
                cache.clone(),
            ),
            simplified_trip_elevation_points_fetcher: SimplifiedTripElevationPointsFetcher::new(
                repos.ride_repo.clone(),
                repos.ride_points_repo.clone(),
                cache,
            ),
            repos: repos.clone(),
            tz_finder: timezone::TimezoneLookup::new(env.get_binding("ASSETS")?),
            jobs: jobs.clone(),
            rwgps_client_id: rwgps.client_id.clone(),
            rwgps_redirect_uri: rwgps.redirect_uri.clone(),
            user_auth_service: user_auth_service.clone(),
        });
        Ok(app_state::AppState {
            schema,
            user_auth_service,
            user_signup_service,
            repos,
            jobs,
            rwgps,
            images: if env
                .var("IMAGES_UPLOADS_ENABLED")
                .ok()
                .map(|v| v.to_string())
                .as_deref()
                == Some("true")
            {
                Some(handlers::media::ImagesConfig::new(
                    env.var("IMAGES_ACCOUNT_ID")?.to_string(),
                    env.var("IMAGES_DELIVERY_HASH")?.to_string(),
                    env.secret("IMAGES_API_TOKEN")?.to_string(),
                )?)
            } else {
                None
            },
        })
    }

    #[event(fetch)]
    async fn fetch(
        req: HttpRequest,
        env: Env,
        _ctx: worker::Context,
    ) -> worker::Result<http::Response<axum::body::Body>> {
        // Acknowledge signed webhooks without opening a database connection. RWGPS
        // expects a response within one second and does not retry failed delivery.
        if matches!(
            (req.method().as_str(), req.uri().path()),
            ("POST", "/webhooks/rwgps")
        ) {
            let rwgps = rwgps_config(&env)
                .map_err(|_| worker::Error::RustError("RWGPS configuration failed".into()))?;
            let state = app_state::RwgpsWebhookState {
                client_secret: rwgps.client_secret,
                jobs: job_queue(&env)
                    .map_err(|_| worker::Error::RustError("Queue configuration failed".into()))?,
            };
            return Ok(webhook_router(state).call(req).await?);
        }
        let state = state(&env)
            .await
            .map_err(|_| worker::Error::RustError("Application initialization failed".into()))?;
        Ok(router(state).call(req).await?)
    }
}
