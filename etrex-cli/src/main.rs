use std::{fs, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use etrex::{trip::detect_trips, EtrexFile};
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
}

#[derive(Args)]
struct Info {
    filepath: PathBuf,
}

#[derive(Args)]
struct Trips {
    dirpath: PathBuf,
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
            let file_paths: Vec<PathBuf> = walkdir::WalkDir::new(&args.dirpath)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| entry.file_type().is_file())
                .map(|entry| entry.path().to_owned())
                .collect();

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
    }

    Ok(())
}
