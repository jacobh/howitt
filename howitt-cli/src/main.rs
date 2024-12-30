#![feature(async_closure)]

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use howitt_fs::{load_huts, load_localities, load_routes, load_stations};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use howitt::services::{
    detect_segments::detect_segments,
    nearby::{nearby_points_of_interest, NearbyPointOfInterest},
};
use rwgps_types::Route;

use crate::json::prettyprintln;

mod description;
mod json;
mod postgres;
mod rwgps;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Stations(Stations),
    Huts(Huts),
    Localities,
    Info(Info),
    #[clap(subcommand)]
    Rwgps(crate::rwgps::Rwgps),
    #[clap(subcommand)]
    Pg(crate::postgres::Postgres),
    #[clap(subcommand)]
    Description(crate::description::Description),
}

#[derive(Args)]
struct GpxInfo {
    filepath: PathBuf,
}

#[derive(Args)]
struct Stations {
    ptv_gtfs_dirpath: PathBuf,
}

#[derive(Args)]
struct Huts {
    filepath: PathBuf,
}

#[derive(Args)]
struct Info {
    route_id: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Stations(_args) => {
            let railway_stations = load_stations()?;
            dbg!(railway_stations.len());
        }
        Commands::Huts(_args) => {
            let huts = load_huts()?;
            dbg!(huts);
        }
        Commands::Localities => {
            load_localities()?;
        }
        Commands::Info(args) => {
            let railway_stations = load_stations()?;
            let huts = load_huts()?;
            let all_pois = railway_stations
                .clone()
                .into_iter()
                .chain(huts.clone().into_iter())
                .collect_vec();
            let routes: Vec<Route> = load_routes()?;

            dbg!(routes.len());
            dbg!(railway_stations.len());
            dbg!(huts.len());

            let routes: Vec<_> = routes
                .into_par_iter()
                .filter(|route| match args.route_id {
                    Some(route_id) => route.id == route_id,
                    None => true,
                })
                .map(|route| {
                    let gpx_route = gpx::Route::from(route.clone());
                    let points = gpx_route.linestring().into_points();

                    let nearby_huts: Vec<_> = nearby_points_of_interest(&points, &huts, 1000.0)
                        .into_iter()
                        .map(NearbyPointOfInterest::into_owned)
                        .collect();
                    let nearby_railway_stations: Vec<_> =
                        nearby_points_of_interest(&points, &railway_stations, 1000.0)
                            .into_iter()
                            .map(NearbyPointOfInterest::into_owned)
                            .collect();

                    let segments = detect_segments(&gpx_route, &all_pois);

                    (route, nearby_huts, nearby_railway_stations, segments)
                })
                .collect();

            for (route, nearby_huts, nearby_railway_stations, segments) in routes {
                if !nearby_huts.is_empty() || !nearby_railway_stations.is_empty() {
                    prettyprintln(serde_json::json!({
                        "route_name": route.name,
                        "huts": nearby_huts
                            .iter()
                            .map(|hut| {
                                serde_json::json!({"hut_name": hut.point_of_interest.name, "distance": hut.distance})
                            })
                            .collect::<Vec<_>>(),
                        "railway_stations": nearby_railway_stations
                            .iter()
                            .map(|railway_station| {
                                serde_json::json!({"railway_station_name": railway_station.point_of_interest.name, "distance": railway_station.distance})
                            })
                            .collect::<Vec<_>>(),
                        "segments": segments.iter().map(|segment| serde_json::json!({
                            "start": segment.start,
                            "end": segment.end,
                        })).collect_vec(),
                    }))
                }
            }
        }
        Commands::Rwgps(command) => crate::rwgps::handle(command).await?,
        Commands::Pg(command) => crate::postgres::handle(command).await?,
        Commands::Description(command) => crate::description::handle(command).await?,
    }

    Ok(())
}
