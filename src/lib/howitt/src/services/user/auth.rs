use chrono::{Months, Utc};
use derive_more::derive::{Constructor, From, Into};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::Serialize;
use thiserror::Error;

use crate::{
    models::user::{UserFilter, UserSession},
    repos::UserRepo,
};

use super::password::verify_password;

#[derive(Debug, Constructor, Clone)]
pub struct UserAuthService {
    user_repo: UserRepo,
    jwt_secret: String,
}

#[derive(Debug, Error)]
#[error("User auth error")]
pub enum UserAuthServiceError {
    UserRepo(anyhow::Error),
    Jwt(#[from] jsonwebtoken::errors::Error),
}

#[derive(Debug, Error)]
pub enum LoginFailed {
    #[error("User with supplied username not found")]
    UsernameNotFound,
    #[error("Password verification failed")]
    PasswordVerificationFailed,
}

#[derive(Debug, From, Into, Serialize, Clone)]
pub struct JwtString(String);

impl JwtString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Login {
    pub session: UserSession,
    pub token: JwtString,
}

impl UserAuthService {
    fn encode_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.jwt_secret.as_bytes())
    }

    fn decode_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.jwt_secret.as_bytes())
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Result<Login, LoginFailed>, UserAuthServiceError> {
        let user = self
            .user_repo
            .find_model(UserFilter::Username(username.to_string()))
            .await
            .map_err(UserAuthServiceError::UserRepo)?;

        match user {
            Some(user) => match verify_password(&user, password) {
                Ok(()) => {
                    let session = UserSession {
                        user_id: user.id,
                        expiry: Utc::now().checked_add_months(Months::new(12)).unwrap(),
                        issued_at: Utc::now(),
                    };

                    let token = JwtString::from(jsonwebtoken::encode(
                        &Header::default(),
                        &session,
                        &self.encode_key(),
                    )?);
                    Ok(Ok(Login { session, token }))
                }
                Err(_) => Ok(Err(LoginFailed::PasswordVerificationFailed)),
            },
            None => Ok(Err(LoginFailed::UsernameNotFound)),
        }
    }

    pub async fn verify(&self, token: &str) -> Result<Login, UserAuthServiceError> {
        let session: UserSession =
            decode(token, &self.decode_key(), &Validation::default())?.claims;

        Ok(Login {
            session,
            token: JwtString::from(token.to_string()),
        })
    }
}
