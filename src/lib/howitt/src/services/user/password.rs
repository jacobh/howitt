use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use thiserror::Error;

use crate::models::user::User;

#[derive(Debug, Error)]
#[error("Password hasher failed")]
pub enum PasswordHashError {
    Argon2(argon2::password_hash::Error),
}

pub fn hash_password(password: &str) -> Result<String, PasswordHashError> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes())
        .map_err(PasswordHashError::Argon2)?;

    Ok(password_hash.to_string())
}

pub fn verify_password(user: &User, password: &str) -> Result<(), PasswordHashError> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    argon2
        .verify_password(password.as_bytes(), user.password.as_str())
        .map_err(PasswordHashError::Argon2)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;
    use crate::models::user::UserId;

    fn user(password: String) -> User {
        User {
            id: UserId::from(Uuid::nil()),
            username: "user".to_owned(),
            password,
            email: "user@example.com".to_owned(),
            created_at: Utc::now(),
            rwgps_connection: None,
        }
    }

    #[test]
    fn hashes_and_verifies_passwords() {
        let user = user(hash_password("correct-password").unwrap());

        assert!(verify_password(&user, "correct-password").is_ok());
        assert!(verify_password(&user, "wrong-password").is_err());
    }

    #[test]
    fn verifies_existing_argon2id_hashes() {
        let user = user(
            "$argon2id$v=19$m=65536,t=2,p=1$c29tZXNhbHQ$\
             CTFhFdXPJO1aFaMaO6Mm5c8y7cJHAph8ArZWb2GRPPc"
                .to_owned(),
        );

        assert!(verify_password(&user, "password").is_ok());
    }
}
