use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    models::user::{User, UserId},
    repos::UserRepo,
    services::user::password::hash_password,
};

#[derive(Debug, Error)]
pub enum UserSignupError {
    #[error("Username already exists")]
    UsernameExists,
    #[error("Email already exists")]
    EmailExists,
    #[error("Password hashing error: {0}")]
    PasswordHashing(#[from] crate::services::user::password::PasswordHashError),
    #[error("Repository error: {0}")]
    Repo(#[from] anyhow::Error),
}

#[derive(Clone)]
pub struct UserSignupService {
    user_repo: UserRepo,
}

impl UserSignupService {
    pub fn new(user_repo: UserRepo) -> Self {
        Self { user_repo }
    }

    pub async fn signup(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, UserSignupError> {
        // Check if username already exists
        if let Some(_) = self
            .user_repo
            .find_model(crate::models::user::UserFilter::Username(username.clone()))
            .await
            .map_err(UserSignupError::Repo)?
        {
            return Err(UserSignupError::UsernameExists);
        }

        // Hash the password
        let password_hash = hash_password(&password)?;

        // Create new user
        let new_user = User {
            id: UserId::from(Uuid::now_v7()),
            username,
            password: password_hash,
            email,
            created_at: Utc::now(),
            rwgps_connection: None,
        };

        // Save the user
        self.user_repo
            .put(new_user.clone())
            .await
            .map_err(UserSignupError::Repo)?;

        Ok(new_user)
    }
}
