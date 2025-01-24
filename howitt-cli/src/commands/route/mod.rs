use clap::{Args, Subcommand};
use description::generate_description;
use howitt::{
    models::route::{RouteFilter, RouteId},
    repos::Repo,
    services::{
        generate_cuesheet::generate_cuesheet,
        simplify_points::{simplify_points, SimplifyTarget},
    },
};
use howitt_postgresql::{PostgresClient, PostgresPointOfInterestRepo, PostgresRouteRepo};
use itertools::Itertools;
use prettytable::{row, Table};
use uuid::Uuid;

mod description;

#[derive(Subcommand)]
pub enum RouteCommands {
    List,
    ListStarred,
    Detail(RouteDetailArgs),
    GenerateCuesheet(RouteDetailArgs),
    GenerateDescription,
}

#[derive(Args)]
pub struct RouteDetailArgs {
    route_id: String,
}

pub async fn handle(command: &RouteCommands) -> Result<(), anyhow::Error> {
    let pg = PostgresClient::connect(
        &std::env::var("DATABASE_URL")
            .unwrap_or(String::from("postgresql://jacob@localhost/howitt")),
    )
    .await?;
    let point_of_interest_repo = PostgresPointOfInterestRepo::new(pg.clone());
    let route_model_repo = PostgresRouteRepo::new(pg.clone());

    match command {
        RouteCommands::GenerateDescription => {
            generate_description();
            Ok(())
        }
        RouteCommands::Detail(args) => {
            let model = route_model_repo
                .get(RouteId::from(Uuid::parse_str(&args.route_id)?))
                .await?;
            dbg!(&model.route);

            let points = model.iter_elevation_points().cloned().collect_vec();
            dbg!(simplify_points(&points, SimplifyTarget::TotalPoints(50)).len());
            Ok(())
        }
        RouteCommands::GenerateCuesheet(args) => {
            let model = route_model_repo
                .get(RouteId::from(Uuid::parse_str(&args.route_id)?))
                .await?;

            let points = model.iter_elevation_points().cloned().collect_vec();
            let pois = point_of_interest_repo.all_indexes().await?;

            let cuesheet = generate_cuesheet(&points, &pois);
            dbg!(cuesheet);
            Ok(())
        }
        RouteCommands::ListStarred => {
            let routes = route_model_repo
                .filter_models(RouteFilter {
                    is_starred: Some(true),
                })
                .await?;

            let mut table = Table::new();
            table.add_row(row!["id", "name", r->"km"]);

            for route in routes {
                let distance_km = route.route.distance / 1000.0;
                table.add_row(row![
                    route.route.id(),
                    route.route.name,
                    r->format!("{distance_km:.1}")
                ]);
            }

            table.printstd();
            Ok(())
        }
        RouteCommands::List => {
            let routes = route_model_repo.all_indexes().await?;
            dbg!(routes);
            Ok(())
        }
    }
}
