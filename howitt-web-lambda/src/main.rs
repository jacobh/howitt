use std::{convert::Infallible, sync::Arc};

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::GraphQLResponse;
use howitt::repos::{CheckpointRepo, ConfigRepo, RideModelRepo, RouteModelRepo};
use howitt_dynamo::SingleTableClient;
use howitt_graphql::Query;
use warp::{http::Response as HttpResponse, Filter};

#[tokio::main]
async fn main() {
    let single_table_client = SingleTableClient::new_from_env().await;

    let config_repo: ConfigRepo =
        Arc::new(howitt_dynamo::ConfigRepo::new(single_table_client.clone()));
    let checkpoint_repo: CheckpointRepo = Arc::new(howitt_dynamo::CheckpointRepo::new(
        single_table_client.clone(),
    ));
    let route_repo: RouteModelRepo = Arc::new(howitt_dynamo::RouteModelRepo::new(
        single_table_client.clone(),
    ));
    let ride_repo: RideModelRepo = Arc::new(howitt_dynamo::RideModelRepo::new(
        single_table_client.clone(),
    ));

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(config_repo)
        .data(checkpoint_repo)
        .data(route_repo)
        .data(ride_repo)
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["content-type"]);

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/graphql").finish())
    });

    let routes = warp::path!("graphiql")
        .and(graphiql)
        .or(warp::path!("graphql").and(graphql_post))
        .with(cors);
    // .recover(|err: Rejection| async move {
    //     if let Some(GraphQLBadRequest(err)) = err.find() {
    //         return Ok::<_, Infallible>(warp::reply::with_status(
    //             err.to_string(),
    //             StatusCode::BAD_REQUEST,
    //         ));
    //     }

    //     Ok(warp::reply::with_status(
    //         "INTERNAL_SERVER_ERROR".to_string(),
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //     ))
    // });

    // Convert them to a warp service (a tower service implmentation)
    // using `warp::service()`
    let warp_service = warp::service(routes);
    // The warp_lambda::run() function takes care of invoking the aws lambda runtime for you
    warp_lambda::run(warp_service)
        .await
        .expect("An error occured");
}
