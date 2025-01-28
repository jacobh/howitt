use anyhow::Result;
use apalis::layers::ErrorHandlingLayer;
use apalis::prelude::*;
use apalis_redis::RedisStorage;
use std::time::Duration;
use tracing::{error, info};

use howitt::jobs::Job;

mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    tracing_subscriber::fmt::init();

    let redis_url = std::env::var("REDIS_URL").unwrap_or("redis://localhost:6379".to_string());
    let conn = apalis_redis::connect(redis_url)
        .await
        .expect("Could not connect");
    let storage: RedisStorage<Job> = RedisStorage::new(conn);

    let worker = WorkerBuilder::new("howitt-worker")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .timeout(Duration::from_millis(30_000))
        .concurrency(4)
        .backend(storage)
        .build_fn(handlers::handle_job);

    Monitor::new()
        .register(worker)
        .on_event(|e| {
            let worker_id = e.id();
            match e.inner() {
                Event::Start => {
                    info!("Worker [{worker_id}] started");
                }
                Event::Error(e) => {
                    error!("Worker [{worker_id}] encountered an error: {e}");
                }

                Event::Exit => {
                    info!("Worker [{worker_id}] exited");
                }
                _ => {}
            }
        })
        .shutdown_timeout(Duration::from_millis(5000))
        .run_with_signal(async {
            info!("Monitor started");
            tokio::signal::ctrl_c().await?;
            info!("Monitor starting shutdown");
            Ok(())
        })
        .await?;
    info!("Monitor shutdown complete");
    Ok(())
}
