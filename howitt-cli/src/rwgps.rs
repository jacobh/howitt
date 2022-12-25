use clap::Subcommand;
use rwgps::{PasswordAuthInfo, TokenAuthInfo, AuthInfo};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{dirs::CONFIG_DIRPATH, json::prettyprintln};

const AUTH_FILENAME: &'static str = "rwgps_auth.toml";

#[derive(Debug, Serialize, Deserialize)]
struct UserConfig {
    password_info: PasswordAuthInfo,
    user_info: Option<UserInfo>
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    user_id: usize,
    token_info: TokenAuthInfo
}

#[derive(Subcommand)]
pub enum Rwgps {
    Info,
    Auth,
}

pub async fn handle(command: &Rwgps) -> Result<(), anyhow::Error> {
    match command {
        Rwgps::Info => {
            println!("hello")
        }
        Rwgps::Auth => {
            let auth_filepath = CONFIG_DIRPATH.join(AUTH_FILENAME);

            let auth_config: UserConfig = match auth_filepath.exists() {
                true => toml::from_slice(&std::fs::read(&auth_filepath)?)?,
                false => {
                    println!("Initial auth setup");
                    let email = inquire::Text::new("Email").prompt();
                    let password = inquire::Password::new("Password")
                        .without_confirmation()
                        .prompt();

                    match (email, password) {
                        (Ok(email), Ok(password)) => UserConfig {
                            password_info: PasswordAuthInfo { email, password },
                            user_info: None
                        },
                        _ => anyhow::bail!("Invalid email/password"),
                    }
                }
            };

            let client = rwgps::RwgpsClient::new();

            let auth_resp = client
                .user_info(&AuthInfo::from(auth_config.password_info.clone()))
                .await?;

            let updated_auth_config = UserConfig {
                user_info: Some(UserInfo { user_id: auth_resp.user.id, token_info: TokenAuthInfo { auth_token: auth_resp.user.auth_token } }),
                ..auth_config
            };

            std::fs::write(auth_filepath, toml::to_vec(&updated_auth_config)?)?;

            prettyprintln(json!({
                "email": updated_auth_config.password_info.email,
                "password": "********",
                "user_info": updated_auth_config.user_info,
            }));
        }
    }

    Ok(())
}
