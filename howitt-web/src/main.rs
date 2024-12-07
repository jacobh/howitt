use std::{convert::Infallible, sync::Arc};

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use howitt::repos::{CachingRepo, ConfigRepo, PointOfInterestRepo, RideModelRepo, RouteModelRepo};
use howitt_dynamo::SingleTableClient;
use howitt_graphql::{
    context::{RequestData, SchemaData},
    credentials::Credentials,
    Query,
};
use warp::{
    http::{Response as HttpResponse, StatusCode},
    Filter, Rejection,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let single_table_client = SingleTableClient::new_from_env().await;

    let config_repo: ConfigRepo = Arc::new(CachingRepo::new(howitt_dynamo::ConfigRepo::new(
        single_table_client.clone(),
    )));
    let poi_repo: PointOfInterestRepo = Arc::new(CachingRepo::new(
        howitt_dynamo::PointOfInterestRepo::new(single_table_client.clone()),
    ));
    let route_repo: RouteModelRepo = Arc::new(CachingRepo::new(
        howitt_dynamo::RouteModelRepo::new(single_table_client.clone()),
    ));
    let ride_repo: RideModelRepo = Arc::new(CachingRepo::new(howitt_dynamo::RideModelRepo::new(
        single_table_client.clone(),
    )));

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(SchemaData {
            config_repo,
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

    let graphql_post = auth_header_filter
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

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}
