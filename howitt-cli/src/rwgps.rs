use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{dirs::CONFIG_DIRPATH, json::prettyprintln};

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

pub async fn handle(command: &Rwgps) -> Result<(), anyhow::Error> {
    match command {
        Rwgps::Info => {
            println!("hello")
        }
        Rwgps::Auth => {
            let auth_filepath = CONFIG_DIRPATH.join(AUTH_FILENAME);

            let auth_config: AuthConfig = match auth_filepath.exists() {
                true => toml::from_slice(&std::fs::read(&auth_filepath)?)?,
                false => {
                    println!("Initial auth setup");
                    let email = inquire::Text::new("Email").prompt();
                    let password = inquire::Password::new("Password")
                        .without_confirmation()
                        .prompt();

                    match (email, password) {
                        (Ok(email), Ok(password)) => AuthConfig {
                            email,
                            password,
                            auth_token: None,
                        },
                        _ => anyhow::bail!("Invalid email/password"),
                    }
                }
            };

            let client = rwgps::RwgpsClient::new();

            let auth_token = client
                .auth(&auth_config.email, &auth_config.password)
                .await?
                .auth_token;

            let updated_auth_config = AuthConfig { auth_token: Some(auth_token), ..auth_config };

            std::fs::write(auth_filepath, toml::to_vec(&updated_auth_config)?)?;

            prettyprintln(json!({
                "email": updated_auth_config.email,
                "password": "********",
                "auth_token": updated_auth_config.auth_token,
            }));
        }
    }

    Ok(())
}
