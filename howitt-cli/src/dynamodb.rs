use std::str::FromStr;

use anyhow::anyhow;
use clap::{Args, Subcommand};
use howitt::{
    models::{
        config::ConfigId,
        external_ref::{ExternalRef, ExternalSource},
        point::ElevationPoint,
        route::{Route, RouteId, RouteModel, RoutePointChunk},
    },
    services::rwgps::RwgpsSyncService,
};
use howitt_dynamo::{
    CheckpointRepo, ConfigRepo, DynamoModelRepo, Keys, RideModelRepo, RouteModelRepo,
    SingleTableClient,
};
use howitt_fs::{load_huts, load_routes, load_stations, load_user_config};
use itertools::Itertools;
use rwgps::RwgpsClient;

#[derive(Subcommand)]
pub enum Dynamodb {
    SyncCheckpoints,
    SyncRoutes,
    SyncRwgps,
    SetStarredRoute(SetStarredRoute),
    ListStarredRoutes,
    GetRoute,
    ListCheckpoints,
    DeleteAll,
}

#[derive(Args)]
pub struct SetStarredRoute {
    route_id: String,
}

pub async fn handle(command: &Dynamodb) -> Result<(), anyhow::Error> {
    let client = SingleTableClient::new_from_env().await;
    let config_repo = ConfigRepo::new(client.clone());
    let checkpoint_repo = CheckpointRepo::new(client.clone());
    let route_model_repo = RouteModelRepo::new(client.clone());
    let ride_model_repo = RideModelRepo::new(client.clone());

    match command {
        Dynamodb::SyncCheckpoints => {
            let stations = load_stations()?;
            let huts = load_huts()?;

            checkpoint_repo.put_batch(stations).await?;
            checkpoint_repo.put_batch(huts).await?;

            println!("done");
        }
        Dynamodb::SyncRoutes => {
            let existing_routes = route_model_repo.all().await?;
            let rwgps_routes = load_routes()?;

            let routes: Vec<_> = rwgps_routes
                .into_iter()
                .map(|route| {
                    let existing_route = existing_routes.iter().find(|existing_route| {
                        existing_route
                            .route
                            .external_ref
                            .as_ref()
                            .map(|ref_| ref_.id == route.id.to_string())
                            .unwrap_or(false)
                    });
                    (route, existing_route)
                })
                .map(|(route, existing_route)| {
                    let id = match existing_route {
                        Some(route) => route.route.id,
                        None => ulid::Ulid::from_datetime(route.created_at.into()),
                    };

                    RouteModel {
                        route: Route {
                            id,
                            name: route.name,
                            distance: route.distance.unwrap_or(0.0),
                            external_ref: Some(ExternalRef {
                                source: ExternalSource::Rwgps,
                                id: route.id.to_string(),
                                updated_at: route.updated_at,
                            }),
                        },
                        point_chunks: route
                            .track_points
                            .into_iter()
                            .filter_map(|track_point| {
                                match (
                                    geo::Point::try_from(track_point.clone()),
                                    track_point.elevation,
                                ) {
                                    (Ok(point), Some(elevation)) => Some((point, elevation)),
                                    _ => None,
                                }
                            })
                            .map(|(point, elevation)| ElevationPoint { point, elevation })
                            .chunks(2500)
                            .into_iter()
                            .enumerate()
                            .map(|(idx, points)| RoutePointChunk {
                                route_id: id,
                                idx,
                                points: points.collect(),
                            })
                            .collect(),
                    }
                })
                .collect();

            route_model_repo.put_batch(routes).await?;
        }
        Dynamodb::SyncRwgps => {
            let config = load_user_config()?.unwrap();
            let rwgps_client = RwgpsClient::new(config.credentials());

            let service = RwgpsSyncService {
                route_repo: route_model_repo,
                ride_repo: ride_model_repo,
                rwgps_client: rwgps_client,
                rwgps_error: std::marker::PhantomData,
            };

            service.sync(config.user_info.unwrap().id).await?;
        }
        Dynamodb::SetStarredRoute(SetStarredRoute { route_id }) => {
            let route_id = ulid::Ulid::from_string(route_id)?;
            let mut config = config_repo.get_model(ConfigId).await?.unwrap_or_default();
            config.starred_route_ids.push(RouteId::from(route_id));
            config_repo.put(config).await?;
        }
        Dynamodb::GetRoute => {
            let model = route_model_repo
                .get_model(RouteId::from(
                    ulid::Ulid::from_str("01GRQGBJ9NNA8354256RQ10DJB").unwrap(),
                ))
                .await?
                .ok_or(anyhow!("route not found"))?;
            dbg!(&model.route.name);
            dbg!(&model.route.external_ref);
            dbg!(model.iter_geo_points().count());
        }
        Dynamodb::ListStarredRoutes => {
            let config = config_repo.get_model(ConfigId).await?.unwrap();

            let routes = route_model_repo.get_batch(config.starred_route_ids).await?;

            for route in routes {
                dbg!(route.route.name);
            }
        }
        Dynamodb::ListCheckpoints => {
            let checkpoints = checkpoint_repo.all().await?;
            dbg!(checkpoints);
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
