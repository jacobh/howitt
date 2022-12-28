use serde::{Deserialize, Serialize};

use crate::credentials::{Credentials, PasswordCredentials};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub password_info: PasswordCredentials,
    pub user_info: Option<crate::types::UserInfo>,
}
impl UserConfig {
    pub fn credentials(&self) -> Credentials {
        match &self.user_info {
            Some(user_info) => Credentials::from_token(user_info.auth_token.clone()),
            None => Credentials::Password(self.password_info.clone()),
        }
    }
}
