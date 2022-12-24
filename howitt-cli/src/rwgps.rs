use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{dirs::config_dirpath, json::prettyprintln};

const AUTH_FILENAME: &'static str = "rwgps_auth.toml";

#[derive(Debug, Serialize, Deserialize)]
struct AuthConfig {
    email: String,
    password: String,
    auth_token: Option<String>,
}

#[derive(Subcommand)]
pub enum Rwgps {
    Info,
    Auth,
}

pub fn handle(command: &Rwgps) -> Result<(), anyhow::Error> {
    match command {
        Rwgps::Info => {
            println!("hello")
        }
        Rwgps::Auth => {
            let auth_filepath = config_dirpath().join(AUTH_FILENAME);

            let auth_config: AuthConfig = match auth_filepath.exists() {
                true => toml::from_slice(&std::fs::read(auth_filepath)?)?,
                false => AuthConfig {
                    email: "".to_string(),
                    password: "".to_string(),
                    auth_token: None,
                },
            };

            prettyprintln(json!({
                "email": auth_config.email,
                "password": "********",
                "auth_token": auth_config.auth_token,
            }));
        }
    }

    Ok(())
}
