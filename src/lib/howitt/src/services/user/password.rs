use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHashString, SaltString, rand_core::OsRng},
};
use thiserror::Error;

use crate::models::user::User;

#[derive(Debug, Error)]
#[error("Password hasher failed")]
pub enum PasswordHashError {
    Argon2(argon2::password_hash::Error),
}

pub fn hash_password(password: &str) -> Result<PasswordHashString, PasswordHashError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(PasswordHashError::Argon2)?;

    Ok(password_hash.serialize())
}

pub fn verify_password(user: &User, password: &str) -> Result<(), PasswordHashError> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    let password_hash = user.password.password_hash();

    argon2
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(PasswordHashError::Argon2)
}
