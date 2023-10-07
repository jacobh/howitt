use std::{convert::identity, str::FromStr};

use clap::{Args, Subcommand, arg};
use howitt::{
    models::{config::ConfigId, route::RouteId},
    repos::Repo,
    services::{generate_cuesheet::generate_cuesheet, rwgps::RwgpsSyncService},
};
use howitt_dynamo::{
    ConfigRepo, Keys, PointOfInterestRepo, RideModelRepo, RouteModelRepo, SingleTableClient,
};
use howitt_fs::{load_huts, load_stations, load_user_config};
use itertools::Itertools;
use prettytable::{row, Table};
use rwgps::RwgpsClient;
use rwgps_types::RouteSummary;

#[derive(Subcommand)]
pub enum Dynamodb {
    SyncPOIs,
    SyncRwgps(SyncRwgps),
    SetStarredRoute(SetStarredRoute),
    ShowConfig,
    ListStarredRoutes,
    ListRoutes,
    GetRoute,
    GenerateCuesheet(GenerateCuesheet),
    ListPOIs,
    DeleteAll,
}

#[derive(Args)]
pub struct SetStarredRoute {
    route_id: String,
}

#[derive(Args)]
pub struct GenerateCuesheet {
    route_id: String,
}

#[derive(Args)]
pub struct SyncRwgps {
    #[arg(long)]
    force_sync_bcs: bool,
    #[arg(long)]
    force_sync_rwgps_id: Option<usize>,
}

pub async fn handle(command: &Dynamodb) -> Result<(), anyhow::Error> {
    let client = SingleTableClient::new_from_env().await;
    let config_repo = ConfigRepo::new(client.clone());
    let point_of_interest_repo = PointOfInterestRepo::new(client.clone());
    let route_model_repo = RouteModelRepo::new(client.clone());
    let ride_model_repo = RideModelRepo::new(client.clone());

    match command {
        Dynamodb::ShowConfig => {
            let config = config_repo.get(ConfigId).await?;
            dbg!(config);
        }
        Dynamodb::SyncPOIs => {
            let stations = load_stations()?;
            let huts = load_huts()?;

            point_of_interest_repo.put_batch(stations).await?;
            point_of_interest_repo.put_batch(huts).await?;

            println!("done");
        }
        Dynamodb::SyncRwgps(SyncRwgps {
            force_sync_bcs,
            force_sync_rwgps_id,
        }) => {
            let config = load_user_config()?.unwrap();
            let rwgps_client = RwgpsClient::new(config.credentials());

            let service = RwgpsSyncService {
                route_repo: route_model_repo,
                ride_repo: ride_model_repo,
                config_repo,
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

            service.sync(config.user_info.unwrap().id).await?;
        }
        Dynamodb::SetStarredRoute(SetStarredRoute { route_id }) => {
            let route_id = ulid::Ulid::from_string(route_id)?;
            let mut config = config_repo.get(ConfigId).await?;
            config.starred_route_ids.push(RouteId::from(route_id));
            config_repo.put(config).await?;
        }
        Dynamodb::GetRoute => {
            let model = route_model_repo
                .get(RouteId::from(
                    ulid::Ulid::from_str("01GRQGBJ9NNA8354256RQ10DJB").unwrap(),
                ))
                .await?;
            dbg!(&model.route.name);
            dbg!(&model.route.external_ref);
            dbg!(model.iter_geo_points().count());
        }
        Dynamodb::GenerateCuesheet(GenerateCuesheet { route_id }) => {
            let model = route_model_repo
                .get(RouteId::from(ulid::Ulid::from_str(route_id).unwrap()))
                .await?;

            let points = model.iter_elevation_points().cloned().collect_vec();
            let pois = point_of_interest_repo.all_indexes().await?;

            let cuesheet = generate_cuesheet(&points, &pois);

            dbg!(cuesheet);
        }
        Dynamodb::ListStarredRoutes => {
            let config = config_repo.get(ConfigId).await?;

            let routes = route_model_repo.get_batch(config.starred_route_ids).await?;

            let mut table = Table::new();

            table.add_row(row!["id", "name", r->"km"]);

            for route in routes {
                let distance_km = route.route.distance / 1000.0;
                table.add_row(
                    row![route.route.id(), route.route.name, r->format!("{distance_km:.1}")],
                );
            }

            table.printstd();
        }
        Dynamodb::ListRoutes => {
            let routes = route_model_repo.all_indexes().await?;
            dbg!(routes);
        }
        Dynamodb::ListPOIs => {
            let pois = point_of_interest_repo.all_indexes().await?;
            dbg!(pois);
        }
        Dynamodb::DeleteAll => {
            let items = client.scan_keys().await?;
            let keys: Vec<Keys> = items
                .iter()
                .map(Keys::from_item)
                .collect::<Result<Vec<_>, _>>()?;
            dbg!(keys.len());

            //// DANGEROUS only uncomment when you want to delete everything

            // let results = keys.into_iter()
            //     .map(|keys| (keys, client.clone()))
            //     .map(async move |(keys, client)| client.delete(keys).await)
            //     .collect::<FuturesUnordered<_>>()
            //     .collect::<Vec<_>>()
            //     .await;

            // dbg!(results);
        }
    }

    Ok(())
}
