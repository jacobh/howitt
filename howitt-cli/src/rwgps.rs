use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Rwgps {
    Info
}

pub fn handle(command: &Rwgps) -> Result<(), anyhow::Error> {
    match command {
        Rwgps::Info => {
            println!("hello")
        }
    }

    Ok(())
}