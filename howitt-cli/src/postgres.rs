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
        Postgres::ListPOIs => {
            let pois = point_of_interest_repo.all_indexes().await?;
            dbg!(pois);
        }
    }

    Ok(())
}
