use argon2::password_hash::PasswordHashString;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Model, ModelName, ModelUuid};

pub type UserId = ModelUuid<{ ModelName::User }>;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: PasswordHashString,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub rwgps_connection: Option<UserRwgpsConnection>,
}

#[derive(Debug, Clone)]
pub struct UserRwgpsConnection {
    pub id: Uuid,
    pub user_id: UserId,
    pub rwgps_user_id: i32,
    pub access_token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum UserFilter {
    Username(String),
    Ids(Vec<UserId>),
}

impl Model for User {
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
