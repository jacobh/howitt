use std::str::FromStr;

use clap::{Args, Subcommand};
use howitt::{
    models::{config::ConfigId, route::RouteId},
    repos::Repo,
    services::rwgps::RwgpsSyncService,
};
use howitt_dynamo::{
    PointOfInterestRepo, ConfigRepo, Keys, RideModelRepo, RouteModelRepo, SingleTableClient,
};
use howitt_fs::{load_huts, load_stations, load_user_config};
use rwgps::RwgpsClient;

#[derive(Subcommand)]
pub enum Dynamodb {
    SyncPOIs,
    SyncRwgps,
    SetStarredRoute(SetStarredRoute),
    ShowConfig,
    ListStarredRoutes,
    ListRoutes,
    GetRoute,
    ListPOIs,
    DeleteAll,
}

#[derive(Args)]
pub struct SetStarredRoute {
    route_id: String,
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
        Dynamodb::SyncRwgps => {
            let config = load_user_config()?.unwrap();
            let rwgps_client = RwgpsClient::new(config.credentials());

            let service = RwgpsSyncService {
                route_repo: route_model_repo,
                ride_repo: ride_model_repo,
                rwgps_client,
                rwgps_error: std::marker::PhantomData,
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
        Dynamodb::ListStarredRoutes => {
            let config = config_repo.get(ConfigId).await?;

            let routes = route_model_repo.get_batch(config.starred_route_ids).await?;

            for route in routes {
                dbg!(route.route.name);
            }
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
