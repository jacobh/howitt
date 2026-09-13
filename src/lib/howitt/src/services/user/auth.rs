use chrono::{DateTime, Utc};
use derive_more::derive::{Constructor, From, Into};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode};
use serde::Serialize;
use thiserror::Error;

use crate::{
    models::user::{UserFilter, UserId, UserSession},
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

    pub fn generate_token(
        &self,
        session: &UserSession,
    ) -> Result<JwtString, jsonwebtoken::errors::Error> {
        Ok(JwtString::from(jsonwebtoken::encode(
            &Header::default(),
            session,
            &self.encode_key(),
        )?))
    }

    pub fn generate_session(
        &self,
        user_id: UserId,
        now: DateTime<Utc>,
        ttl: chrono::Duration,
    ) -> UserSession {
        UserSession {
            user_id,
            expiry: now.checked_add_signed(ttl).unwrap(),
            issued_at: now,
        }
    }

    pub fn generate_login(
        &self,
        user_id: UserId,
        now: DateTime<Utc>,
        ttl: chrono::Duration,
    ) -> Result<Login, jsonwebtoken::errors::Error> {
        let session = self.generate_session(user_id, now, ttl);
        let token = self.generate_token(&session)?;
        Ok(Login { session, token })
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
                Ok(()) => Ok(Ok(self.generate_login(
                    user.id,
                    Utc::now(),
                    chrono::Duration::days(365),
                )?)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use jsonwebtoken::errors::ErrorKind;
    use uuid::Uuid;

    const SECRET: &[u8] = b"compatibility-test-secret";
    const V9_TOKEN: &str = concat!(
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.",
        "eyJzdWIiOiJVU0VSIzAxMjM0NTY3LTg5YWItN2RlZi04MTIzLTQ1Njc4OWFiY2RlZiIs",
        "ImV4cCI6NDEwMjQ0NDgwMCwiaWF0IjoxNzA2OTMzMTA2fQ.",
        "Nm8SZnbEx52Srk5iT6XVVs7gtdfGfHIbzI49n5tkKwE",
    );

    fn compatible_session() -> UserSession {
        UserSession {
            user_id: UserId::from(Uuid::from_u128(0x01234567_89ab_7def_8123_456789abcdef)),
            expiry: Utc.timestamp_opt(4_102_444_800, 0).unwrap(),
            issued_at: Utc.timestamp_opt(1_706_933_106, 0).unwrap(),
        }
    }

    #[test]
    fn jwt_v9_hs256_tokens_remain_compatible() {
        let key = DecodingKey::from_secret(SECRET);
        let decoded = decode::<UserSession>(V9_TOKEN, &key, &Validation::default()).unwrap();

        assert_eq!(decoded.claims.user_id, compatible_session().user_id);
        assert_eq!(decoded.claims.expiry, compatible_session().expiry);
        assert_eq!(decoded.claims.issued_at, compatible_session().issued_at);
        assert_eq!(
            jsonwebtoken::encode(
                &Header::default(),
                &compatible_session(),
                &EncodingKey::from_secret(SECRET),
            )
            .unwrap(),
            V9_TOKEN,
        );
    }

    #[test]
    fn jwt_validation_still_rejects_invalid_signatures_and_expired_tokens() {
        let invalid_signature = decode::<UserSession>(
            V9_TOKEN,
            &DecodingKey::from_secret(b"wrong-secret"),
            &Validation::default(),
        )
        .unwrap_err();
        assert!(matches!(
            invalid_signature.kind(),
            ErrorKind::InvalidSignature
        ));

        let mut expired_session = compatible_session();
        expired_session.expiry = Utc.timestamp_opt(1_577_836_800, 0).unwrap();
        let expired_token = jsonwebtoken::encode(
            &Header::default(),
            &expired_session,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();
        let expired = decode::<UserSession>(
            &expired_token,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        )
        .unwrap_err();
        assert!(matches!(expired.kind(), ErrorKind::ExpiredSignature));
    }
}
