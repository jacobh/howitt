use serde::{Deserialize, Serialize};

#[derive(derive_more::From, Debug, Clone, Serialize, Deserialize)]
pub enum Credentials {
    Password(PasswordCredentials),
    Token(TokenCredentials),
}
impl Credentials {
    pub fn from_token(auth_token: String) -> Credentials {
        Credentials::Token(TokenCredentials { auth_token })
    }
    pub fn to_query(&self) -> serde_json::Value {
        match self {
            Credentials::Password(info) => serde_json::json!({
                "email": info.email,
                "password": info.password,
                "apikey": "howitt",
                "version": "2"
            }),
            Credentials::Token(info) => serde_json::json!({
                "auth_token": info.auth_token,
                "apikey": "howitt",
                "version": "2"
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCredentials {
    pub auth_token: String,
}
