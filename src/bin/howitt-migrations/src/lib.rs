#[cfg(any(target_arch = "wasm32", test))]
fn authorized(header: Option<&str>, secret: &str) -> bool {
    use sha2::{Digest, Sha256};
    use subtle::ConstantTimeEq;

    let Some(token) = header.and_then(|header| header.strip_prefix("Bearer ")) else {
        return false;
    };
    let supplied = Sha256::digest(token.as_bytes());
    let expected = Sha256::digest(secret.as_bytes());
    bool::from(supplied.ct_eq(&expected))
}

#[cfg(target_arch = "wasm32")]
mod runtime {
    use howitt_postgresql::{
        MigrationError, PostgresPool, baseline_migrations, bundled_migrations, run_migrations,
    };
    use serde::Deserialize;
    use worker::{Context, Env, Request, Response, Result, event};

    #[derive(Deserialize)]
    struct BaselineRequest {
        through: i64,
        confirmation: String,
    }

    fn error_response(error: MigrationError) -> Result<Response> {
        let (status, message) = match error {
            MigrationError::UnknownVersion(_)
            | MigrationError::NameDrift { .. }
            | MigrationError::ChecksumDrift { .. }
            | MigrationError::InvalidOrder(_)
            | MigrationError::HistoryGap { .. }
            | MigrationError::BaselineHistoryNotEmpty
            | MigrationError::InvalidBaseline(_) => (409, error.to_string()),
            MigrationError::Database { .. } => {
                let (phase, version, sqlstate) = error
                    .database_diagnostic()
                    .expect("database error has a diagnostic");
                worker::console_error!(
                    "migration.failed kind=database phase={} version={} sqlstate={}",
                    phase,
                    version
                        .map(|version| format!("V{version:04}"))
                        .unwrap_or_else(|| "none".to_owned()),
                    sqlstate.unwrap_or("none")
                );
                let message = match (phase, version, sqlstate) {
                    ("commit", _, _) => {
                        "Database migration commit failed; outcome is unknown".to_owned()
                    }
                    (_, Some(version), Some(sqlstate)) => format!(
                        "Database migration failed at V{version:04} during {phase} (SQLSTATE {sqlstate})"
                    ),
                    (_, Some(version), None) => {
                        format!("Database migration failed at V{version:04} during {phase}")
                    }
                    (_, None, Some(sqlstate)) => {
                        format!("Database migration failed during {phase} (SQLSTATE {sqlstate})")
                    }
                    (_, None, None) => format!("Database migration failed during {phase}"),
                };
                (500, message)
            }
            MigrationError::Repository(_) => {
                worker::console_error!(
                    "migration.failed kind=database phase=acquire version=none sqlstate=none"
                );
                (500, "Database connection acquisition failed".to_owned())
            }
        };
        Response::error(message, status)
    }

    #[event(fetch)]
    async fn fetch(mut request: Request, env: Env, _context: Context) -> Result<Response> {
        let secret = env.secret("MIGRATION_ADMIN_TOKEN")?.to_string();
        let authorization = request.headers().get("Authorization")?;
        if !super::authorized(authorization.as_deref(), &secret) {
            return Response::error("Unauthorized", 401);
        }
        if request.method() != worker::Method::Post {
            return Response::error("Method not allowed", 405);
        }

        let pool = PostgresPool::from_hyperdrive(env.hyperdrive("HYPERDRIVE")?)
            .map_err(|_| worker::Error::RustError("Database configuration failed".into()))?;
        match request.path().as_str() {
            "/apply" => match run_migrations(&pool, bundled_migrations()).await {
                Ok(result) => {
                    worker::console_log!(
                        "migration.completed action=apply applied_count={}",
                        result.applied_versions.len()
                    );
                    Response::from_json(&serde_json::json!({
                        "status": "ok",
                        "appliedVersions": result.applied_versions,
                    }))
                }
                Err(error) => error_response(error),
            },
            "/baseline" => {
                let body = match request.json::<BaselineRequest>().await {
                    Ok(body) => body,
                    Err(_) => return Response::error("Invalid request", 400),
                };
                let expected = format!("BASELINE EXISTING SCHEMA THROUGH V{:04}", body.through);
                if body.confirmation != expected {
                    return Response::error("Baseline confirmation does not match", 400);
                }
                match baseline_migrations(&pool, bundled_migrations(), body.through).await {
                    Ok(result) => {
                        worker::console_log!(
                            "migration.completed action=baseline recorded_count={} through={}",
                            result.recorded_versions.len(),
                            body.through
                        );
                        Response::from_json(&serde_json::json!({
                            "status": "ok",
                            "baselinedVersions": result.recorded_versions,
                        }))
                    }
                    Err(error) => error_response(error),
                }
            }
            _ => Response::error("Not found", 404),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn authorization_requires_an_exact_bearer_token() {
        assert!(super::authorized(
            Some("Bearer correct-secret"),
            "correct-secret"
        ));
        assert!(!super::authorized(None, "correct-secret"));
        assert!(!super::authorized(Some("correct-secret"), "correct-secret"));
        assert!(!super::authorized(
            Some("Bearer wrong-secret"),
            "correct-secret"
        ));
        assert!(!super::authorized(
            Some("Bearer correct-secret-extra"),
            "correct-secret"
        ));
    }
}
