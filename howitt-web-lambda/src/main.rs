use std::{convert::Infallible, sync::Arc};

use async_graphql::{Schema, EmptyMutation, EmptySubscription, http::GraphiQLSource};
use async_graphql_warp::{GraphQLResponse, GraphQLBadRequest};
use howitt::{config::Config, checkpoint::Checkpoint, repo::{CheckpointRepo, RouteRepo}};
use howitt_dynamo::SingleTableClient;
use howitt_graphql::Query;
use http::StatusCode;
use warp::{Filter, http::Response as HttpResponse, Rejection};

#[tokio::main]
async fn main() {
    let routes: Vec<rwgps::types::Route> = vec![];
    let trips: Vec<howitt::trip::EtrexTrip> = vec![];
    let config = Config {
        starred_route_ids: vec![],
    };
    let huts: Vec<Checkpoint> = vec![];
    let stations: Vec<Checkpoint> = vec![];
    let all_checkpoints: Vec<Checkpoint> = vec![];

    let single_table_client = SingleTableClient::new_from_env().await;

    let checkpoint_repo: CheckpointRepo = Arc::new(howitt_dynamo::CheckpointRepo::new(single_table_client.clone()));
    let route_repo: RouteRepo = Arc::new(howitt_dynamo::RouteRepo::new(single_table_client.clone()));

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(config)
        .data(routes)
        .data(all_checkpoints)
        .data(trips)
        .data(checkpoint_repo)
        .data(route_repo)
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

    let routes = warp::path!("graphiql").and(graphiql)
        .or(warp::path!("graphql").and(graphql_post));
        // .with(cors)
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