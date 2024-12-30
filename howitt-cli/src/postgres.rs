use std::{convert::identity, str::FromStr};

use clap::{arg, Args, Subcommand};
use howitt::{
    models::{point::simplify_points, route::RouteId},
    repos::Repo,
    services::{
        generate_cuesheet::generate_cuesheet,
        sync::{photo::PhotoSyncService, rwgps::RwgpsSyncService},
    },
};
use howitt_clients::{ReqwestHttpClient, S3BucketClient};
use howitt_fs::{load_huts, load_stations, load_user_config};
use howitt_postgresql::{
    PostgresClient, PostgresPointOfInterestRepo, PostgresRideRepo, PostgresRouteRepo,
};
use itertools::Itertools;
use rwgps::RwgpsClient;
use rwgps_types::RouteSummary;

#[derive(Subcommand)]
pub enum Postgres {
    SyncPOIs,
    SyncRwgps(SyncRwgps),
    SyncPhotos,
    ListStarredRoutes,
    ListRoutes,
    GetRoute(RouteIdArgs),
    GenerateCuesheet(RouteIdArgs),
    ListPOIs,
}

#[derive(Args)]
pub struct RouteIdArgs {
    route_id: String,
}

#[derive(Args)]
pub struct SyncRwgps {
    #[arg(long)]
    force_sync_bcs: bool,
    #[arg(long)]
    force_sync_rwgps_id: Option<usize>,
}

pub async fn handle(command: &Postgres) -> Result<(), anyhow::Error> {
    let pg = PostgresClient::connect("postgres://jacob@localhost/howitt").await?;
    let point_of_interest_repo = PostgresPointOfInterestRepo::new(pg.clone());
    let route_model_repo = PostgresRouteRepo::new(pg.clone());
    let ride_model_repo = PostgresRideRepo::new(pg.clone());

    match command {
        Postgres::SyncPOIs => {
            let stations = load_stations()?;
            let huts = load_huts()?;

            point_of_interest_repo.put_batch(stations).await?;
            point_of_interest_repo.put_batch(huts).await?;

            println!("done");
        }
        Postgres::SyncRwgps(SyncRwgps {
            force_sync_bcs,
            force_sync_rwgps_id,
        }) => {
            let config = load_user_config()?.unwrap();
            let rwgps_client = RwgpsClient::new(config.credentials());

            let service = RwgpsSyncService {
                route_repo: route_model_repo,
                ride_repo: ride_model_repo,
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
        Postgres::SyncPhotos => {
            let photo_sync = PhotoSyncService {
                bucket_client: S3BucketClient::new_from_env(
                    howitt_client_types::BucketName::Photos,
                )
                .await,
                http_client: ReqwestHttpClient::new(),
                route_repo: route_model_repo,
            };

            let result = photo_sync.sync().await;

            dbg!(&result);

            result?
        }
        Postgres::GetRoute(RouteIdArgs { route_id }) => {
            let model = route_model_repo
                .get(RouteId::from(ulid::Ulid::from_str(&route_id).unwrap()))
                .await?;
            dbg!(&model.route);

            let points = model.iter_elevation_points().cloned().collect_vec();

            dbg!(simplify_points(&points, 50).len());
        }
        Postgres::GenerateCuesheet(RouteIdArgs { route_id }) => {
            let model = route_model_repo
                .get(RouteId::from(ulid::Ulid::from_str(route_id).unwrap()))
                .await?;

            let points = model.iter_elevation_points().cloned().collect_vec();
            let pois = point_of_interest_repo.all_indexes().await?;

            let cuesheet = generate_cuesheet(&points, &pois);

            dbg!(cuesheet);
        }
        Postgres::ListStarredRoutes => {
            unimplemented!()
            // let routes = route_model_repo.get_batch(config.starred_route_ids).await?;

            // let mut table = Table::new();

            // table.add_row(row!["id", "name", r->"km"]);

            // for route in routes {
            //     let distance_km = route.route.distance / 1000.0;
            //     table.add_row(
            //         row![route.route.id(), route.route.name, r->format!("{distance_km:.1}")],
            //     );
            // }

            // table.printstd();
        }
        Postgres::ListRoutes => {
            let routes = route_model_repo.all_indexes().await?;
            dbg!(routes);
        }
        Postgres::ListPOIs => {
            let pois = point_of_interest_repo.all_indexes().await?;
            dbg!(pois);
        }
    }

    Ok(())
}
