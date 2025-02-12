use chrono_tz::Australia::Melbourne;
use clap::{Args, Subcommand};
use howitt::{
    repos::AnyhowRepo,
    services::simplify_points::{simplify_points_v2, DetailLevel},
};
use howitt_postgresql::PostgresRepos;
use serde_json::json;

use crate::Context;

#[derive(Subcommand)]
pub enum RideCommands {
    List,
    Detail(RideDetailArgs),
    PreviewPoints(RideDetailArgs),
}

#[derive(Args)]
pub struct RideDetailArgs {
    ride_id: String,
}

pub async fn handle(
    command: &RideCommands,
    Context {
        repos: PostgresRepos {
            ride_points_repo, ..
        },
        ..
    }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        RideCommands::PreviewPoints(RideDetailArgs { ride_id }) => {
            let ride_id = howitt::models::ride::RideId::from(uuid::Uuid::parse_str(ride_id)?);
            let ride_points = ride_points_repo.get(ride_id).await?;

            let simplified = simplify_points_v2(ride_points.points, DetailLevel::ExtremelyLow);

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

            Ok(())
        }
        _ => Ok(()), // Placeholder - implement actual handlers
    }
}
