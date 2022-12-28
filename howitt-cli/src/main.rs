#![feature(async_closure)]

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::{Args, Parser, Subcommand};
use gtfs::GtfsZip;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use ::rwgps::types::Route;
use howitt::{checkpoint::Checkpoint, nearby::nearby_checkpoints, trip::detect_trips, EtrexFile};

use crate::json::prettyprintln;

mod dirs;
mod json;
mod rwgps;

struct Config {
    ptv_gtfs_dirpath: &'static str,
    huts_filepath: &'static str,
}

const CONFIG: Config = Config {
    ptv_gtfs_dirpath: "../data/ptv_gtfs",
    huts_filepath: "../data/HUTS.gpx",
};

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
    Info,
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

fn find_file_paths(dirpath: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(dirpath)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_owned())
        .collect()
}

fn load_stations(ptv_gtfs_dirpath: &Path) -> Result<Vec<Checkpoint>, anyhow::Error> {
    let file_paths: Vec<PathBuf> = find_file_paths(ptv_gtfs_dirpath)
        .into_iter()
        .filter(|path| path.extension() == Some("zip".as_ref()))
        .collect();

    let gtfs_zips: Vec<GtfsZip> = file_paths
        .into_iter()
        .map(|path| -> Result<_, anyhow::Error> {
            let data = fs::read(path)?;
            let cursor = io::Cursor::new(data);
            Ok(GtfsZip::parse(cursor)?)
        })
        .collect::<Result<_, _>>()?;

    let checkpoints = gtfs_zips
        .into_iter()
        .flat_map(|zip| zip.stops)
        .map(Checkpoint::from);
    Ok(checkpoints
        .filter(|checkpoint| checkpoint.name.contains("Railway Station"))
        .collect::<Vec<_>>())
}

fn load_huts(filepath: &Path) -> Result<Vec<Checkpoint>, anyhow::Error> {
    let data = fs::read(filepath)?;
    let file = EtrexFile::parse(&data)?;
    Ok(file
        .gpx
        .waypoints
        .into_iter()
        .map(Checkpoint::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

fn load_routes() -> Result<Vec<Route>, anyhow::Error> {
    let data = fs::read(dirs::CONFIG_DIRPATH.join("rwgps_routes.json"))?;
    Ok(serde_json::from_slice(&data)?)
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
        Commands::Stations(args) => {
            let railway_stations = load_stations(&args.ptv_gtfs_dirpath)?;
            dbg!(railway_stations);
        }
        Commands::Huts(args) => {
            let huts = load_huts(&args.filepath)?;
            dbg!(huts);
        }
        Commands::Info => {
            let railway_stations = load_stations(CONFIG.ptv_gtfs_dirpath.as_ref())?;
            let huts = load_huts(CONFIG.huts_filepath.as_ref())?;
            let routes: Vec<Route> = load_routes()?;

            dbg!(routes.len());
            dbg!(railway_stations.len());
            dbg!(huts.len());

            let routes: Vec<_> = routes
                .into_par_iter()
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

                    (route, nearby_huts, nearby_railway_stations)
                })
                .collect();

            for (route, nearby_huts, nearby_railway_stations) in routes {
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
                    }))
                }
            }
        }
        Commands::Rwgps(command) => crate::rwgps::handle(command).await?,
    }

    Ok(())
}
