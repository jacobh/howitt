use std::{
    fs, io,
    path::{Path, PathBuf},
};

use gtfs::GtfsZip;
use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};

use howitt::{checkpoint::Checkpoint, config::Config, EtrexFile};
use project_root::get_project_root;
use shapefile::{dbase::FieldValue, record::polygon::GenericPolygon, Point, PolygonRing};

use geo::{centroid::Centroid, LineString, Polygon};

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

pub fn load_config() -> Result<Config, anyhow::Error> {
    let data = fs::read(&get_project_root()?.join("data/config.toml"))?;
    Ok(toml::from_slice(&data)?)
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

pub fn load_localities() -> Result<Vec<Checkpoint>, anyhow::Error> {
    let mut reader = shapefile::Reader::from_path(
        &get_project_root()?.join("data/vic_localities/vic_localities.shp"),
    )?;

    let checkpoints = reader
        .iter_shapes_and_records()
        .map(|shape_record| -> Result<Checkpoint, anyhow::Error> {
            let (shape, record) = shape_record?;

            let name = record
                .get("LOC_NAME")
                .map(|x| match x {
                    FieldValue::Character(x) => x.as_ref(),
                    _ => None,
                })
                .unwrap_or_default()
                .unwrap()
                .clone();

            let polygon = convert_polygon(shapefile::Polygon::try_from(shape)?);

            Ok(Checkpoint {
                id: uuid::Uuid::new_v4(),
                name,
                point: polygon.centroid().unwrap(),
                checkpoint_type: howitt::checkpoint::CheckpointType::Locality,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(checkpoints)
}

fn ring_to_linestring(ring: &PolygonRing<Point>) -> LineString<f64> {
    geo::LineString::from_iter(ring.points().into_iter().map(|point| (point.x, point.y)))
}

fn convert_polygon(polygon: GenericPolygon<Point>) -> Polygon<f64> {
    let exterior = polygon
        .rings()
        .iter()
        .find(|ring| match ring {
            PolygonRing::Outer(_) => true,
            _ => false,
        })
        .unwrap();
    let interiors = polygon.rings().iter().filter(|ring| match ring {
        PolygonRing::Inner(_) => true,
        _ => false,
    });

    Polygon::new(
        ring_to_linestring(exterior),
        interiors.map(ring_to_linestring).collect(),
    )
}
