use std::{convert::identity, sync::Arc};

use chrono::Utc;
use chrono_tz::Australia::Melbourne;
use clap::{arg, Args, Subcommand};
use howitt::{
    models::{
        route::RouteId,
        user::{User, UserId},
    },
    repos::Repo,
    services::{
        generate_cuesheet::generate_cuesheet,
        simplify_points::{simplify_points, SimplifyTarget},
        sync::{
            photo::PhotoSyncService,
            rwgps::{RwgpsSyncService, SyncParams},
        },
        user::{auth::UserAuthService, password::hash_password},
    },
};
use howitt_clients::{ReqwestHttpClient, S3BucketClient};
use howitt_fs::{load_huts, load_stations, load_user_config};
use howitt_postgresql::{
    PostgresClient, PostgresPointOfInterestRepo, PostgresRidePointsRepo, PostgresRideRepo,
    PostgresRouteRepo, PostgresUserRepo,
};
use itertools::Itertools;
use rwgps::RwgpsClient;
use rwgps_types::RouteSummary;
use serde_json::json;

#[derive(Subcommand)]
pub enum Postgres {
    ListStarredRoutes,
    ListRoutes,
    GetRoute(RouteIdArgs),
    GenerateCuesheet(RouteIdArgs),
    PreviewRidePoints(RideIdArgs),
    ListPOIs,
}

#[derive(Args)]
pub struct RouteIdArgs {
    route_id: String,
}

#[derive(Args)]
pub struct RideIdArgs {
    ride_id: String,
}

#[derive(Args)]
pub struct VerifyTokenArgs {
    token: String,
}

pub async fn handle(command: &Postgres) -> Result<(), anyhow::Error> {
    let pg = PostgresClient::connect(
        &std::env::var("DATABASE_URL")
            .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
    )
    .await?;
    let point_of_interest_repo = PostgresPointOfInterestRepo::new(pg.clone());
    let route_model_repo = PostgresRouteRepo::new(pg.clone());
    let ride_repo = PostgresRideRepo::new(pg.clone());
    let ride_points_repo = PostgresRidePointsRepo::new(pg.clone());
    let user_repo = PostgresUserRepo::new(pg.clone());

    match command {
        Postgres::GetRoute(RouteIdArgs { route_id }) => {
            let model = route_model_repo
                .get(RouteId::from(uuid::Uuid::parse_str(route_id).unwrap()))
                .await?;
            dbg!(&model.route);

            let points = model.iter_elevation_points().cloned().collect_vec();

            dbg!(simplify_points(&points, SimplifyTarget::TotalPoints(50)).len());
        }
        Postgres::GenerateCuesheet(RouteIdArgs { route_id }) => {
            let model = route_model_repo
                .get(RouteId::from(uuid::Uuid::parse_str(route_id).unwrap()))
                .await?;

            let points = model.iter_elevation_points().cloned().collect_vec();
            let pois = point_of_interest_repo.all_indexes().await?;

            let cuesheet = generate_cuesheet(&points, &pois);

            dbg!(cuesheet);
        }
        Postgres::PreviewRidePoints(RideIdArgs { ride_id }) => {
            let ride_id = howitt::models::ride::RideId::from(uuid::Uuid::parse_str(ride_id)?);
            let ride_points = ride_points_repo.get(ride_id).await?;

            let simplified = simplify_points(&ride_points.points, SimplifyTarget::TotalPoints(25));

            // Convert to [[lng, lat, elevation_m, timestamp], ...] format
            let preview_points: Vec<Vec<serde_json::Value>> = simplified
                .iter()
                .map(|point| {
                    vec![
                        json!(point.point.x()),
                        json!(point.point.y()),
                        json!(point.elevation),
                        json!(point.datetime.with_timezone(&Melbourne).to_rfc3339()),
                    ]
                })
                .collect();

            println!("{}", serde_json::to_string_pretty(&preview_points)?);
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
