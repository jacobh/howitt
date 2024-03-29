use clap::{Args, Subcommand};
use howitt::ext::futures::FuturesIteratorExt;
use howitt_fs::{
    load_routes, load_user_config, persist_routes, persist_trips, persist_user_config,
};
use itertools::Itertools;
use rwgps_types::{config::UserConfig, credentials::PasswordCredentials};
use serde_json::json;

use crate::json::prettyprintln;

#[derive(Subcommand)]
pub enum Rwgps {
    Info,
    Auth,
    Sync,
    #[clap(subcommand)]
    Routes(Routes),
    Trips,
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
        Rwgps::Sync => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            let route_summaries = client
                .user_routes(user_config.user_info.as_ref().unwrap().id)
                .await?;

            let routes: Vec<Result<rwgps_types::Route, _>> = route_summaries
                .into_iter()
                .map(|route| (route, client.clone()))
                .map(async move |(route, client)| client.route(route.id).await)
                .collect_futures_ordered()
                .await;

            let routes = routes.into_iter().collect::<Result<Vec<_>, _>>()?;

            persist_routes(&routes)?;
            dbg!(routes.len());

            let trip_summaries = client
                .user_trips(user_config.user_info.as_ref().unwrap().id)
                .await?;

            let trips: Vec<Result<rwgps_types::Trip, _>> = trip_summaries
                .into_iter()
                .map(|trip| (trip, client.clone()))
                .map(async move |(trip, client)| client.trip(trip.id).await)
                .collect_futures_ordered()
                .await;

            let trips: Vec<rwgps_types::Trip> = trips.into_iter().collect::<Result<Vec<_>, _>>()?;

            persist_trips(&trips)?;
            dbg!(trips.len());
        }
        Rwgps::Routes(Routes::List) => {
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
        Rwgps::Routes(Routes::Detail(args)) => {
            let user_config = get_user_config()?;
            let client = rwgps::RwgpsClient::new(user_config.credentials());

            let resp = client.route(args.route_id).await?;
            dbg!(resp);
        }
        Rwgps::Trips => {
            unimplemented!()
        }
    }

    Ok(())
}
