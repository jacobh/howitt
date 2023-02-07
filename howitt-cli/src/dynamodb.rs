use clap::Subcommand;
use futures::{prelude::*, stream::FuturesUnordered};
use howitt::{
    external_ref::{ExternalRef, ExternalSource},
    point::ElevationPoint,
    route::Route,
};
use howitt_dynamo::{CheckpointRepo, DynamoRepo, RouteRepo, SingleTableClient};
use howitt_fs::{load_huts, load_routes};

#[derive(Subcommand)]
pub enum Dynamodb {
    Sync,
    SyncRoutes,
    ListCheckpoints,
}

pub async fn handle(command: &Dynamodb) -> Result<(), anyhow::Error> {
    let client = SingleTableClient::new_from_env().await;
    let checkpoint_repo = CheckpointRepo::new(client.clone());
    let route_repo = RouteRepo::new(client.clone());

    match command {
        Dynamodb::Sync => {
            let huts = load_huts()?;

            checkpoint_repo.put_batch(huts).await?;

            println!("done");
        }
        Dynamodb::SyncRoutes => {
            let existing_routes = route_repo.all().await?;
            let rwgps_routes = load_routes()?;

            let routes: Vec<_> = rwgps_routes
                .into_iter()
                .map(|route| {
                    let existing_route = existing_routes.iter().find(|existing_route| {
                        existing_route
                            .external_ref
                            .as_ref()
                            .map(|ref_| ref_.id == route.id.to_string())
                            .unwrap_or(false)
                    });
                    (route, existing_route)
                })
                .map(|(route, existing_route)| Route {
                    id: existing_route.map(|route| route.id).unwrap_or(ulid::Ulid::new()),
                    name: route.name,
                    distance: route.distance.unwrap_or(0.0),
                    points: route
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
                        .collect(),
                    external_ref: Some(ExternalRef {
                        source: ExternalSource::Rwgps,
                        id: route.id.to_string(),
                        updated_at: route.updated_at,
                    }),
                })
                .collect();

            route_repo.put_batch(routes).await?;
        }
        Dynamodb::ListCheckpoints => {
            let checkpoints = checkpoint_repo.all().await?;
            dbg!(checkpoints);
        }
    }

    Ok(())
}
