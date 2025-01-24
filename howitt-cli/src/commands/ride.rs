use chrono_tz::Australia::Melbourne;
use clap::{Args, Subcommand};
use howitt::{
    repos::AnyhowRepo,
    services::simplify_points::{simplify_points, SimplifyTarget},
};
use howitt_postgresql::{PostgresClient, PostgresRidePointsRepo};
use serde_json::json;

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

pub async fn handle(command: &RideCommands) -> Result<(), anyhow::Error> {
    let pg = PostgresClient::connect(
        &std::env::var("DATABASE_URL")
            .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
    )
    .await?;
    let ride_points_repo = PostgresRidePointsRepo::new(pg.clone());
    match command {
        RideCommands::PreviewPoints(RideDetailArgs { ride_id }) => {
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

            Ok(())
        }
        _ => Ok(()), // Placeholder - implement actual handlers
    }
}
