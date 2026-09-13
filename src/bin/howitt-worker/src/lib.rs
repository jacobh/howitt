pub mod handlers;

// Classify failures without logging error bodies, URLs, or database values.
#[cfg(any(target_arch = "wasm32", test))]
fn failure_kind(error: &anyhow::Error) -> &'static str {
    if error.to_string() == "User has no RWGPS connection" {
        return "missing_rwgps_connection";
    }
    if let Some(error) = error.downcast_ref::<rwgps::RwgpsError>() {
        return match error {
            rwgps::RwgpsError::Reqwest(_) => "rwgps_request",
            rwgps::RwgpsError::Url(_) => "rwgps_url",
            rwgps::RwgpsError::SerdeDebug(_) => "rwgps_response",
        };
    }
    if error
        .downcast_ref::<howitt_postgresql::PostgresRepoError>()
        .is_some()
    {
        return "database";
    }
    "processing"
}

#[cfg(target_arch = "wasm32")]
mod runtime {
    use howitt::{jobs::QueueMessage, repos::Repos};
    use howitt_postgresql::{PostgresPool, PostgresRepos, PostgresRwgpsSyncStore};
    use std::sync::Arc;
    use worker::{event, Context, Env, MessageBatch, MessageExt, Result};

    #[event(queue)]
    async fn queue(batch: MessageBatch<serde_json::Value>, env: Env, _ctx: Context) -> Result<()> {
        for decoded in batch.iter() {
            // A malformed SDK envelope fails the invocation rather than silently
            // acknowledging an unreadable message. Explicit earlier acks survive.
            let message = decoded?;
            let payload = serde_json::from_value::<QueueMessage>(message.body().clone());
            let Ok(QueueMessage::V1(job)) = payload else {
                worker::console_error!("job.invalid message_id={}", message.id());
                message.retry(); // exhaust configured retries into the dead-letter queue
                continue;
            };
            let mut phase = "initialize";
            let result = async {
                // Connections belong to this invocation, never isolate globals.
                let pool =
                    PostgresPool::from_hyperdrive(env.hyperdrive("HYPERDRIVE")?).map_err(|_| {
                        worker::Error::RustError("Database configuration failed".into())
                    })?;
                let repos = Repos::from(PostgresRepos::new(pool.clone()));
                phase = "process";
                let children = super::handlers::handle_job(
                    job,
                    repos,
                    rwgps::RwgpsClient::new(),
                    Arc::new(PostgresRwgpsSyncStore::new(pool)),
                )
                .await
                .map_err(|error| {
                    worker::console_error!(
                        "job.failure_kind message_id={} kind={}",
                        message.id(),
                        super::failure_kind(&error)
                    );
                    worker::Error::RustError("Job processing failed".into())
                })?;
                phase = "publish";
                for child in children {
                    howitt_jobs::enqueue(&env, child).await?;
                }
                Ok::<_, worker::Error>(())
            }
            .await;
            match result {
                Ok(()) => {
                    message.ack();
                    worker::console_log!("job.completed message_id={}", message.id());
                }
                Err(_) => {
                    // Avoid recording credentials, upstream bodies, or SQL values.
                    worker::console_error!(
                        "job.failed message_id={} phase={}",
                        message.id(),
                        phase
                    );
                    message.retry();
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn failure_kind_does_not_include_error_details() {
        assert_eq!(
            super::failure_kind(&anyhow::anyhow!("User has no RWGPS connection")),
            "missing_rwgps_connection"
        );
        assert_eq!(
            super::failure_kind(&anyhow::anyhow!("secret-token")),
            "processing"
        );
        let error = howitt_postgresql::PostgresRepoError::Connection("secret-host".into());
        assert_eq!(super::failure_kind(&anyhow::Error::new(error)), "database");
    }
}
