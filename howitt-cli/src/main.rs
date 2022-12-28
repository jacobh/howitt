#![feature(async_closure)]

use std::{fs, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use howitt_fs::{find_file_paths, load_huts, load_routes, load_stations};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use ::rwgps::types::Route;
use howitt::{nearby::nearby_checkpoints, segment::detect_segments, trip::detect_trips, EtrexFile};

use crate::json::prettyprintln;

mod json;
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
    GpxInfo(GpxInfo),
    Trips(Trips),
    Stations(Stations),
    Huts(Huts),
    Info(Info),
    #[clap(subcommand)]
    Rwgps(crate::rwgps::Rwgps),
}

#[derive(Args)]
struct GpxInfo {
    filepath: PathBuf,
}

#[derive(Args)]
struct Trips {
    dirpath: PathBuf,
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
        Commands::GpxInfo(args) => {
            let data = fs::read(&args.filepath)?;
            let file = EtrexFile::parse(&data)?;
            dbg!(&file);
        }
        Commands::Trips(args) => {
            let file_paths: Vec<PathBuf> = find_file_paths(&args.dirpath);

            let files: Vec<EtrexFile> = file_paths
                .into_par_iter()
                .map(|path| -> Result<_, anyhow::Error> {
                    let data = fs::read(path)?;
                    Ok(EtrexFile::parse(&data)?)
                })
                .collect::<Result<_, _>>()?;

            let trips: Vec<_> = detect_trips(files);
            dbg!(&trips.len());
            for trip in trips {
                dbg!(trip);
            }
        }
        Commands::Stations(_args) => {
            let railway_stations = load_stations()?;
            dbg!(railway_stations.len());
        }
        Commands::Huts(_args) => {
            let huts = load_huts()?;
            dbg!(huts);
        }
        Commands::Info(args) => {
            let railway_stations = load_stations()?;
            let huts = load_huts()?;
            let all_checkpoints = railway_stations
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
                    let nearby_huts: Vec<_> = nearby_checkpoints(&gpx_route, &huts)
                        .into_iter()
                        .filter(|checkpoint| checkpoint.distance < 1000.0)
                        .collect();
                    let nearby_railway_stations: Vec<_> =
                        nearby_checkpoints(&gpx_route, &railway_stations)
                            .into_iter()
                            .filter(|checkpoint| checkpoint.distance < 1000.0)
                            .collect();

                    let segments = detect_segments(&gpx_route, &all_checkpoints);

                    (route, nearby_huts, nearby_railway_stations, segments)
                })
                .collect();

            for (route, nearby_huts, nearby_railway_stations, segments) in routes {
                if nearby_huts.len() > 0 || nearby_railway_stations.len() > 0 {
                    prettyprintln(serde_json::json!({
                        "route_name": route.name,
                        "huts": nearby_huts
                            .iter()
                            .map(|hut| {
                                serde_json::json!({"hut_name": hut.checkpoint.name, "distance": hut.distance})
                            })
                            .collect::<Vec<_>>(),
                        "railway_stations": nearby_railway_stations
                            .iter()
                            .map(|railway_station| {
                                serde_json::json!({"railway_station_name": railway_station.checkpoint.name, "distance": railway_station.distance})
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
    }

    Ok(())
}
