use clap::{Args, Subcommand};
use futures::{prelude::*, StreamExt};
use rwgps::credentials::{Credentials, PasswordCredentials};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{dirs::CONFIG_DIRPATH, json::prettyprintln};

const CONFIG_FILENAME: &'static str = "rwgps_auth.toml";

#[derive(Debug, Serialize, Deserialize)]
struct UserConfig {
    password_info: PasswordCredentials,
    user_info: Option<rwgps::types::UserInfo>,
}
impl UserConfig {
    fn credentials(&self) -> Credentials {
        match &self.user_info {
            Some(user_info) => Credentials::from_token(user_info.auth_token.clone()),
            None => Credentials::Password(self.password_info.clone()),
        }
    }
}

#[derive(Subcommand)]
pub enum Rwgps {
    Info,
    Auth,
    #[clap(subcommand)]
    Routes(Routes),
}

#[derive(Subcommand)]
pub enum Routes {
    List,
    Sync,
    Detail(RouteDetailArgs),
}

#[derive(Args)]
pub struct RouteDetailArgs {
    route_id: usize,
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
                    password_info: PasswordCredentials { email, password },
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
    match command {
        Rwgps::Info => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            dbg!(client.user_info().await?);
        }
        Rwgps::Auth => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            let auth_resp = client.user_info().await?;

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
        Rwgps::Routes(Routes::List) => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            let routes = client
                .user_routes(user_config.user_info.unwrap().id)
                .await?;

            // println!("{}", serde_json::to_string_pretty(&resp)?);

            dbg!(routes.len());
        }
        Rwgps::Routes(Routes::Detail(args)) => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            let resp = client.route(args.route_id).await?;
            dbg!(resp);
        }
        Rwgps::Routes(Routes::Sync) => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            let routes = client
                .user_routes(user_config.user_info.unwrap().id)
                .await?;

            let futs = routes
                .into_iter()
                .map(|route| (route, client.clone()))
                .map(async move |(route, client)| client.route(route.id).await);

            let x: Vec<Result<rwgps::types::Route, _>> = futures::stream::iter(futs).buffer_unordered(20).collect::<Vec<_>>().await;

            dbg!(x.len());
        }
    }

    Ok(())
}
