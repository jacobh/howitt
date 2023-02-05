use std::convert::Infallible;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use howitt::config::Config;
use howitt_fs::{load_config, load_huts, load_routes, load_stations, load_trips};
use http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};
use howitt_graphql::Query;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let routes: Vec<rwgps::types::Route> = load_routes()?;
    let trips = load_trips()?;
    let config: Config = load_config()?;
    let huts = load_huts()?;
    let stations = load_stations()?;
    let all_checkpoints: Vec<_> = huts.into_iter().chain(stations).collect();

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(config)
        .data(routes)
        .data(all_checkpoints)
        .data(trips)
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
