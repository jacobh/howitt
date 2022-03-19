use std::{path::PathBuf, fs};

use clap::{Args, Parser, Subcommand};
use etrex::EtrexFile;

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
}

#[derive(Args)]
struct Info {
    filepath: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Info(args) => {
            let data = fs::read(&args.filepath).expect("Unable to read file");
            let file = EtrexFile::new(data);
            dbg!(&file);
        }
    }
}