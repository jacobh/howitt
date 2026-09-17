pub mod handlers;

// Classify failures without logging error bodies, URLs, or database values.
#[cfg(any(target_arch = "wasm32", test))]
fn failure_diagnostic(error: &anyhow::Error) -> String {
    if let Some(error) = error.downcast_ref::<rwgps::RwgpsError>() {
        return match error {
            rwgps::RwgpsError::Reqwest(_) => "kind=rwgps_request".into(),
            rwgps::RwgpsError::Url(_) => "kind=rwgps_url".into(),
            rwgps::RwgpsError::SerdeDebug(error) => {
                format!("kind=rwgps_response {}", error.safe_diagnostic())
            }
        };
    }
    if error.to_string() == "User has no RWGPS connection" {
        return "kind=missing_rwgps_connection".into();
    }
    if error
        .downcast_ref::<howitt_postgresql::PostgresRepoError>()
        .is_some()
    {
        return "kind=database".into();
    }
    "kind=processing".into()
}

#[cfg(target_arch = "wasm32")]
mod runtime {
    use howitt::{jobs::QueueMessage, repos::Repos};
    use howitt_postgresql::{PostgresPool, PostgresRepos, PostgresRwgpsSyncStore};
    use std::sync::Arc;
    use worker::{Context, Env, MessageBatch, MessageExt, Result, event};

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
                        "job.failure_kind message_id={} {}",
                        message.id(),
                        super::failure_diagnostic(&error)
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
    fn failure_diagnostic_does_not_include_error_details() {
        assert_eq!(
            super::failure_diagnostic(&anyhow::anyhow!("User has no RWGPS connection")),
            "kind=missing_rwgps_connection"
        );
        assert_eq!(
            super::failure_diagnostic(&anyhow::anyhow!("secret-token")),
            "kind=processing"
        );
        let error = howitt_postgresql::PostgresRepoError::Connection("secret-host".into());
        assert_eq!(
            super::failure_diagnostic(&anyhow::Error::new(error)),
            "kind=database"
        );
    }
}
