#![feature(async_closure)]
use std::{convert::Infallible, sync::Arc};

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use auth::login_route;
use howitt_postgresql::{
    PostgresClient, PostgresPointOfInterestRepo, PostgresRideRepo, PostgresRouteRepo,
    PostgresUserRepo,
};
use warp::{
    http::{Response as HttpResponse, StatusCode},
    Filter, Rejection,
};

mod auth;
mod graphql;

use graphql::{
    context::{RequestData, SchemaData},
    credentials::Credentials,
    Query,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pg = PostgresClient::connect(
        &std::env::var("DATABASE_URL")
            .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
    )
    .await?;

    let poi_repo = Arc::new(PostgresPointOfInterestRepo::new(pg.clone()));
    let route_repo = Arc::new(PostgresRouteRepo::new(pg.clone()));
    let ride_repo = Arc::new(PostgresRideRepo::new(pg.clone()));
    let user_repo = Arc::new(PostgresUserRepo::new(pg.clone()));

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(SchemaData {
            poi_repo,
            route_repo,
            ride_repo,
        })
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["content-type", "authorization"]);

    let auth_header_filter =
        warp::header::optional::<String>("authorization").map(|auth_header: Option<String>| {
            auth_header
                .as_deref()
                .and_then(|s| Credentials::parse_auth_header_value(s).ok())
        });

    let graphql_post = warp::path::end()
        .and(auth_header_filter)
        .and(async_graphql_warp::graphql(schema))
        .and_then(
            |credentials,
             (schema, mut request): (
                Schema<Query, EmptyMutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                request = request.data(RequestData { credentials });
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/").finish())
    });

    let routes = graphiql
        .or(graphql_post)
        .or(login_route(user_repo))
        .with(cors)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes.with(warp::compression::gzip()))
        .run(([0, 0, 0, 0], 8000))
        .await;

    Ok(())
}
