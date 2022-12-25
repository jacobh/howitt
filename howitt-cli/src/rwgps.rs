use clap::Subcommand;
use rwgps::{AuthInfo, PasswordAuthInfo, TokenAuthInfo};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{dirs::CONFIG_DIRPATH, json::prettyprintln};

const CONFIG_FILENAME: &'static str = "rwgps_auth.toml";

#[derive(Debug, Serialize, Deserialize)]
struct UserConfig {
    password_info: PasswordAuthInfo,
    user_info: Option<rwgps::UserInfo>,
}

#[derive(Subcommand)]
pub enum Rwgps {
    Info,
    Auth,
}

fn get_user_config() -> Result<UserConfig, anyhow::Error> {
    let config_filepath = CONFIG_DIRPATH.join(CONFIG_FILENAME);

    match config_filepath.exists() {
        true => Ok(toml::from_slice(&std::fs::read(&config_filepath)?)?),
        false => {
            println!("Initial user setup");
            let email = inquire::Text::new("Email").prompt();
            let password = inquire::Password::new("Password")
                .without_confirmation()
                .prompt();

            match (email, password) {
                (Ok(email), Ok(password)) => Ok(UserConfig {
                    password_info: PasswordAuthInfo { email, password },
                    user_info: None,
                }),
                _ => anyhow::bail!("Invalid email/password"),
            }
        }
    }
}

fn persist_user_config(config: &UserConfig) -> Result<(), anyhow::Error> {
    let config_filepath = CONFIG_DIRPATH.join(CONFIG_FILENAME);

    std::fs::write(config_filepath, toml::to_vec(config)?)?;

    Ok(())
}

pub async fn handle(command: &Rwgps) -> Result<(), anyhow::Error> {
    let client = rwgps::RwgpsClient::new();

    match command {
        Rwgps::Info => {
            let user_config = get_user_config()?;

            match user_config.user_info {
                Some(user_info) => {
                    let resp = client.user_info(&AuthInfo::from_token(user_info.auth_token)).await?;
                    dbg!(resp);
                }
                None => {}
            }
        }
        Rwgps::Auth => {
            let user_config = get_user_config()?;

            let auth_resp = client
                .user_info(&AuthInfo::from(user_config.password_info.clone()))
                .await?;

            let updated_user_config = UserConfig {
                user_info: Some(auth_resp.user),
                ..user_config
            };

            persist_user_config(&updated_user_config)?;

            prettyprintln(json!({
                "email": updated_user_config.password_info.email,
                "password": "********",
                "user_info": updated_user_config.user_info,
            }));
        }
    }

    Ok(())
}
