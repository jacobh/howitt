#![feature(async_closure)]
use howitt::ext::futures::FuturesIteratorExt;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {}

async fn function_handler(_args: LambdaEvent<Request>) -> Result<Response, Error> {
    let target_domains = std::env::var("TARGET_DOMAINS").unwrap();
    let target_urls = target_domains
        .split(',')
        .map(|domain| format!("https://{domain}"));

    let client = reqwest::Client::new();

    let resps = target_urls
        .into_iter()
        .map(|url| (url, client.clone()))
        .map(async move |(url, client)| client.get(url).send().await)
        .collect_futures_ordered()
        .await;

    dbg!(&resps);

    // resps.unwrap();

    Ok(Response {})
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
