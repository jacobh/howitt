use std::{
    fs,
    path::{Path, PathBuf}, io,
};

use clap::{Args, Parser, Subcommand};
use etrex::{gtfs::GtfsZip, trip::detect_trips, EtrexFile, checkpoint::Checkpoint};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Info(Info),
    Trips(Trips),
    Stations(Stations),
}

#[derive(Args)]
struct Info {
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

fn find_file_paths(dirpath: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(dirpath)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_owned())
        .collect()
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Info(args) => {
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
            let file_paths: Vec<PathBuf> = find_file_paths(&args.ptv_gtfs_dirpath)
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

            let checkpoints = gtfs_zips.into_iter().flat_map(|zip| zip.stops).map(Checkpoint::from);
            let railway_stations = checkpoints.filter(|checkpoint| checkpoint.name.contains("Railway Station")).collect::<Vec<_>>();
            dbg!(railway_stations);
        }
    }

    Ok(())
}
