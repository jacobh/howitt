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
use itertools::Itertools;
use prettytable::{row, Table};
use uuid::Uuid;

use crate::Context;

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

pub async fn handle(
    command: &RouteCommands,
    Context {
        route_repo,
        poi_repo,
        ..
    }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        RouteCommands::GenerateDescription => {
            generate_description();
            Ok(())
        }
        RouteCommands::Detail(args) => {
            let model = route_repo
                .get(RouteId::from(Uuid::parse_str(&args.route_id)?))
                .await?;
            dbg!(&model.route);

            let points = model.iter_elevation_points().cloned().collect_vec();
            dbg!(simplify_points(&points, SimplifyTarget::TotalPoints(50)).len());
            Ok(())
        }
        RouteCommands::GenerateCuesheet(args) => {
            let model = route_repo
                .get(RouteId::from(Uuid::parse_str(&args.route_id)?))
                .await?;

            let points = model.iter_elevation_points().cloned().collect_vec();
            let pois = poi_repo.all_indexes().await?;

            let cuesheet = generate_cuesheet(&points, &pois);
            dbg!(cuesheet);
            Ok(())
        }
        RouteCommands::ListStarred => {
            let routes = route_repo.filter_models(RouteFilter::Starred).await?;

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
            let routes = route_repo.all_indexes().await?;
            dbg!(routes);
            Ok(())
        }
    }
}
