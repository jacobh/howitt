use clap::Subcommand;
use futures::{prelude::*, stream::FuturesUnordered};
use howitt::{
    external_ref::{ExternalRef, ExternalSource},
    point::ElevationPoint,
    route::{Route, RouteModel, RoutePointChunk},
};
use howitt_dynamo::{CheckpointRepo, DynamoModelRepo, RouteModelRepo, SingleTableClient};
use howitt_fs::{load_huts, load_routes};
use itertools::Itertools;

#[derive(Subcommand)]
pub enum Dynamodb {
    Sync,
    SyncRoutes,
    GetRoute,
    ListCheckpoints,
}

pub async fn handle(command: &Dynamodb) -> Result<(), anyhow::Error> {
    let client = SingleTableClient::new_from_env().await;
    let checkpoint_repo = CheckpointRepo::new(client.clone());
    let route_model_repo = RouteModelRepo::new(client.clone());

    match command {
        Dynamodb::Sync => {
            let huts = load_huts()?;

            checkpoint_repo.put_batch(huts).await?;

            println!("done");
        }
        Dynamodb::SyncRoutes => {
            // let existing_routes = route_repo.all().await?;
            let rwgps_routes = load_routes()?;

            let routes: Vec<_> = rwgps_routes
                .into_iter()
                // .map(|route| {
                //     let existing_route = existing_routes.iter().find(|existing_route| {
                //         existing_route
                //             .external_ref
                //             .as_ref()
                //             .map(|ref_| ref_.id == route.id.to_string())
                //             .unwrap_or(false)
                //     });
                //     (route, existing_route)
                // })
                .map(|route| {
                    let id = ulid::Ulid::new();

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
        Dynamodb::GetRoute => {
            let model = route_model_repo
                .get_model("01GRQGBJ9NNA8354256RQ10DJB".to_string())
                .await?;
            dbg!(&model.route.name);
            dbg!(&model.route.external_ref);
            dbg!(model.iter_geo_points().count());
        }
        Dynamodb::ListCheckpoints => {
            let checkpoints = checkpoint_repo.all().await?;
            dbg!(checkpoints);
        }
    }

    Ok(())
}
