pub mod handlers;

#[cfg(any(target_arch = "wasm32", test))]
fn job_operation(job: &howitt::jobs::Job) -> &'static str {
    use howitt::jobs::{Job, media::MediaJob, rwgps::RwgpsJob};

    match job {
        Job::Media(MediaJob::InferLocation(_)) => "media_infer_location",
        Job::Rwgps(RwgpsJob::Webhook(_)) => "rwgps_webhook",
        Job::Rwgps(RwgpsJob::SyncTrip { .. }) => "rwgps_sync_trip",
        Job::Rwgps(RwgpsJob::SyncRoute { .. }) => "rwgps_sync_route",
        Job::Rwgps(RwgpsJob::SyncHistory { .. }) => "rwgps_sync_history",
    }
}

// Classify failures without logging error bodies, URLs, or database values.
#[cfg(any(target_arch = "wasm32", test))]
fn failure_diagnostic(error: &anyhow::Error, operation: &'static str) -> String {
    let kind = if let Some(error) = error.downcast_ref::<rwgps::RwgpsError>() {
        match error {
            rwgps::RwgpsError::Reqwest(_) => "rwgps_request",
            rwgps::RwgpsError::Url(_) => "rwgps_url",
            rwgps::RwgpsError::SerdeDebug(error) => {
                return format!(
                    "kind=rwgps_response operation={operation} {}",
                    error.safe_diagnostic()
                );
            }
        }
    } else if error.to_string() == "User has no RWGPS connection" {
        "missing_rwgps_connection"
    } else if error.to_string() == "No user found for RWGPS webhook" {
        "missing_rwgps_webhook_user"
    } else if matches!(
        error.to_string().as_str(),
        "RWGPS route belongs to another user" | "RWGPS trip belongs to another user"
    ) {
        "rwgps_ownership_conflict"
    } else if error
        .downcast_ref::<howitt::services::sync::rwgps_v2::sync_trip::InsufficientUsableTripPoints>()
        .is_some()
    {
        "rwgps_trip_no_usable_points"
    } else if let Some(error) = error
        .downcast_ref::<howitt::services::sync::rwgps_v2::persistence::RwgpsSyncPersistenceError>()
    {
        use howitt::services::sync::rwgps_v2::persistence::RwgpsSyncPersistenceErrorKind;

        match error.kind() {
            RwgpsSyncPersistenceErrorKind::Database => "rwgps_persistence_database",
            RwgpsSyncPersistenceErrorKind::SerializationConversion => {
                "rwgps_persistence_serialization_conversion"
            }
            RwgpsSyncPersistenceErrorKind::OwnershipConflict => "rwgps_ownership_conflict",
        }
    } else if error
        .downcast_ref::<howitt::services::media::MediaGeoInferrerError>()
        .is_some()
    {
        "media_missing_captured_at"
    } else if error
        .downcast_ref::<howitt_postgresql::PostgresRepoError>()
        .is_some()
    {
        "database"
    } else {
        "processing"
    };

    format!("kind={kind} operation={operation}")
}

#[cfg(any(target_arch = "wasm32", test))]
fn is_permanent_failure(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<howitt::services::sync::rwgps_v2::sync_trip::InsufficientUsableTripPoints>()
        .is_some()
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
            let operation = super::job_operation(&job);
            let mut phase = "initialize";
            let mut permanent_failure = false;
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
                    permanent_failure = super::is_permanent_failure(&error);
                    worker::console_error!(
                        "job.failure_kind message_id={} {}",
                        message.id(),
                        super::failure_diagnostic(&error, operation)
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
                    if permanent_failure {
                        message.ack();
                        worker::console_log!(
                            "job.permanent_failure_acknowledged message_id={} phase={}",
                            message.id(),
                            phase
                        );
                    } else {
                        worker::console_error!(
                            "job.failed message_id={} phase={}",
                            message.id(),
                            phase
                        );
                        message.retry();
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use howitt::{
        jobs::{Job, media::MediaJob, rwgps::RwgpsJob},
        models::{media::MediaId, user::UserId},
        services::sync::rwgps_v2::persistence::RwgpsSyncPersistenceError,
    };

    fn assert_redacted(diagnostic: &str) {
        for sensitive in [
            "secret-token",
            "private response value",
            "select * from users",
            "https://private.example/users/42",
            "USER#00000000-0000-0000-0000-000000000042",
            "987654",
        ] {
            assert!(!diagnostic.contains(sensitive), "leaked {sensitive}");
        }
    }

    #[test]
    fn failure_diagnostic_classifies_known_processing_failures() {
        assert_eq!(
            super::failure_diagnostic(
                &anyhow::anyhow!("User has no RWGPS connection"),
                "rwgps_sync_route"
            ),
            "kind=missing_rwgps_connection operation=rwgps_sync_route"
        );
        assert_eq!(
            super::failure_diagnostic(
                &anyhow::Error::new(
                    howitt::services::sync::rwgps_v2::sync_trip::InsufficientUsableTripPoints {
                        usable_points: 1,
                    }
                ),
                "rwgps_sync_trip"
            ),
            "kind=rwgps_trip_no_usable_points operation=rwgps_sync_trip"
        );
        assert_eq!(
            super::failure_diagnostic(
                &anyhow::anyhow!("RWGPS route belongs to another user"),
                "rwgps_sync_route"
            ),
            "kind=rwgps_ownership_conflict operation=rwgps_sync_route"
        );
        let error = howitt_postgresql::PostgresRepoError::Connection("secret-host".into());
        assert_eq!(
            super::failure_diagnostic(&anyhow::Error::new(error), "rwgps_sync_history"),
            "kind=database operation=rwgps_sync_history"
        );
    }

    #[test]
    fn only_insufficient_trip_points_are_permanent() {
        let insufficient = anyhow::Error::new(
            howitt::services::sync::rwgps_v2::sync_trip::InsufficientUsableTripPoints {
                usable_points: 0,
            },
        );
        assert!(super::is_permanent_failure(&insufficient));
        assert!(!super::is_permanent_failure(&anyhow::anyhow!(
            "unrelated trip processing failure"
        )));
        assert!(!super::is_permanent_failure(&anyhow::Error::new(
            RwgpsSyncPersistenceError::database(std::io::Error::other("database failure"))
        )));
    }

    #[test]
    fn failure_diagnostic_classifies_persistence_causes_without_source_values() {
        let hostile_source = || {
            std::io::Error::other(
                "secret-token private response value select * from users \
                 https://private.example/users/42 \
                 USER#00000000-0000-0000-0000-000000000042 987654",
            )
        };
        let cases = [
            (
                RwgpsSyncPersistenceError::database(hostile_source()),
                "kind=rwgps_persistence_database operation=rwgps_sync_route",
            ),
            (
                RwgpsSyncPersistenceError::serialization_conversion(hostile_source()),
                "kind=rwgps_persistence_serialization_conversion operation=rwgps_sync_route",
            ),
            (
                RwgpsSyncPersistenceError::ownership_conflict(hostile_source()),
                "kind=rwgps_ownership_conflict operation=rwgps_sync_route",
            ),
        ];

        for (error, expected) in cases {
            let diagnostic =
                super::failure_diagnostic(&anyhow::Error::new(error), "rwgps_sync_route");
            assert_eq!(diagnostic, expected);
            assert_redacted(&diagnostic);
        }
    }

    #[test]
    fn failure_diagnostic_reports_operation_without_unknown_error_or_job_values() {
        let diagnostic = super::failure_diagnostic(
            &anyhow::anyhow!(
                "secret-token private response value select * from users \
                 https://private.example/users/42 \
                 USER#00000000-0000-0000-0000-000000000042 987654"
            ),
            "rwgps_sync_trip",
        );
        assert_eq!(diagnostic, "kind=processing operation=rwgps_sync_trip");
        assert_redacted(&diagnostic);
    }

    #[test]
    fn job_operation_uses_only_the_variant() {
        let media = Job::Media(MediaJob::InferLocation(MediaId::new()));
        let route = Job::Rwgps(RwgpsJob::SyncRoute {
            rwgps_route_id: 987654,
            user_id: UserId::new(),
        });

        assert_eq!(super::job_operation(&media), "media_infer_location");
        assert_eq!(super::job_operation(&route), "rwgps_sync_route");
        assert_redacted(super::job_operation(&route));
    }
}
