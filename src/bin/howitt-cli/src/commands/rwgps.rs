use std::convert::identity;

use clap::{arg, Args, Subcommand};
use howitt::{
    models::user::UserId,
    services::sync::rwgps::{RwgpsSyncService, SyncParams},
};
use howitt_fs::load_user_config;
use howitt_fs::{load_routes, persist_user_config};
use itertools::Itertools;
use rwgps::RwgpsClient;
use rwgps_types::RouteSummary;
use rwgps_types::{config::UserConfig, credentials::PasswordCredentials};
use serde_json::json;

use crate::utils::json::prettyprintln;
use crate::Context;

#[derive(Subcommand)]
pub enum RwgpsCommands {
    Info,
    Auth,
    #[clap(subcommand)]
    Routes(Routes),
    Trips,
    Sync(SyncRwgps),
}

#[derive(Args)]
pub struct SyncRwgps {
    #[arg(long)]
    force_sync_bcs: bool,
    #[arg(long)]
    force_sync_rwgps_id: Option<usize>,
}

#[derive(Subcommand)]
pub enum Routes {
    List,
    Detail(RouteDetailArgs),
}

#[derive(Args)]
pub struct RouteDetailArgs {
    route_id: usize,
}

fn get_user_config() -> Result<UserConfig, anyhow::Error> {
    let config = load_user_config()?;

    match config {
        Some(config) => Ok(config),
        None => {
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

pub async fn handle(
    command: &RwgpsCommands,
    Context {
        ride_points_repo,
        ride_repo,
        route_repo,
        ..
    }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        RwgpsCommands::Info => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            dbg!(client.user_info().await?);
        }
        RwgpsCommands::Auth => {
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
        RwgpsCommands::Sync(SyncRwgps {
            force_sync_bcs,
            force_sync_rwgps_id,
        }) => {
            let config = load_user_config()?.unwrap();
            let rwgps_client = RwgpsClient::new(config.credentials());

            let service = RwgpsSyncService {
                route_repo,
                ride_repo,
                ride_points_repo,
                rwgps_client,
                rwgps_error: std::marker::PhantomData,
                should_force_sync_route_fn: Some(|summary: &RouteSummary| {
                    [
                        *force_sync_bcs && summary.name.contains("[BCS]"),
                        force_sync_rwgps_id
                            .map(|id| id == summary.id)
                            .unwrap_or(false),
                    ]
                    .into_iter()
                    .any(identity)
                }),
            };

            service
                .sync(SyncParams {
                    rwgps_user_id: config.user_info.unwrap().id,
                    user_id: UserId::from(uuid::Uuid::parse_str(
                        "01941a60-9cfd-c166-94bb-126a6d8de5fd",
                    )?),
                })
                .await?;
        }
        RwgpsCommands::Routes(Routes::List) => {
            let routes: Vec<rwgps_types::Route> = load_routes()?
                .into_iter()
                .sorted_by_key(|route| route.id)
                .collect_vec();

            let rows = vec![prettytable::row![
                "id",
                "name",
                "distance (km)",
                "last modified"
            ]]
            .into_iter()
            .chain(routes.into_iter().map(|route| {
                prettytable::row![
                    route.id,
                    route.name,
                    route.distance.unwrap_or(0.0) / 1000.0,
                    route.updated_at,
                ]
            }))
            .collect_vec();

            prettytable::Table::init(rows).printstd()
        }
        RwgpsCommands::Routes(Routes::Detail(args)) => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            let resp = client.route(args.route_id).await?;
            dbg!(resp);
        }
        RwgpsCommands::Trips => {
            unimplemented!()
        }
    }

    Ok(())
}
