use std::{
    fs, io,
    path::{Path, PathBuf},
};

use gtfs::GtfsZip;
use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};

use howitt::{checkpoint::Checkpoint, EtrexFile};
use project_root::get_project_root;

mod dirs;
mod rwgps;

pub use crate::rwgps::*;

pub fn find_file_paths(dirpath: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(dirpath)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_owned())
        .collect()
}

pub fn load_stations() -> Result<Vec<Checkpoint>, anyhow::Error> {
    let file_paths: Vec<PathBuf> = find_file_paths(&get_project_root()?.join("data/ptv_gtfs"))
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
        .sorted_by_key(|stop| stop.stop_id.clone())
        .dedup_by(|stop1, stop2| stop1.stop_id == stop2.stop_id)
        .map(Checkpoint::from);

    Ok(checkpoints
        .filter(|checkpoint| checkpoint.name.contains("Railway Station"))
        .collect::<Vec<_>>())
}

pub fn load_huts() -> Result<Vec<Checkpoint>, anyhow::Error> {
    let data = fs::read(&get_project_root()?.join("data/HUTS.gpx"))?;
    let file = EtrexFile::parse(&data)?;
    Ok(file
        .gpx
        .waypoints
        .into_iter()
        .map(|mut waypoint| {
            waypoint._type = Some("HUT".to_string());
            waypoint
        })
        .map(Checkpoint::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}
