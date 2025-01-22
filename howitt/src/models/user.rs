use argon2::password_hash::PasswordHashString;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{IndexModel, ModelName, ModelUuid};

pub type UserId = ModelUuid<{ ModelName::User }>;

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

#[derive(Debug, Clone)]
pub struct UserFilter {
    pub username: Option<String>,
}

impl IndexModel for User {
    type Id = UserId;
    type Filter = UserFilter;

    fn id(&self) -> UserId {
        UserId::from(self.id)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
    #[serde(rename = "sub")]
    pub user_id: UserId,
    #[serde(rename = "exp", with = "ts_seconds")]
    pub expiry: DateTime<Utc>,
    #[serde(rename = "iat", with = "ts_seconds")]
    pub issued_at: DateTime<Utc>,
}
