use argon2::{
    password_hash::{rand_core::OsRng, PasswordHashString, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{IndexModel, ModelName, ModelUlid};

pub type UserId = ModelUlid<{ ModelName::User }>;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: PasswordHashString,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub linked_accounts: Vec<LinkedAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkedAccount {
    Rwgps(usize),
}

impl IndexModel for User {
    type Id = UserId;
    type Filter = ();

    fn id(&self) -> UserId {
        UserId::from(self.id)
    }
}

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
