use std::{
    fs, io,
    path::{Path, PathBuf},
};

use gtfs::{GtfsStop, GtfsZip};
use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};

use chrono::prelude::*;
use howitt::ext::ulid::generate_ulid;
use howitt::models::{
    config::Config,
    point_of_interest::{PointOfInterest, PointOfInterestType},
};
use project_root::get_project_root;
use shapefile::{Point, PolygonRing, dbase::FieldValue, record::polygon::GenericPolygon};

use geo::{LineString, Polygon, centroid::Centroid};

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
    let data = fs::read(get_project_root()?.join("data/config.toml"))?;
    Ok(toml::from_str(&String::from_utf8(data)?)?)
}

pub fn load_stations() -> Result<Vec<PointOfInterest>, anyhow::Error> {
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

    let pois = gtfs_zips
        .into_iter()
        .flat_map(|zip| zip.stops)
        .sorted_by_key(|stop| stop.stop_id.clone())
        .dedup_by(|stop1, stop2| stop1.stop_id == stop2.stop_id)
        .map(|stop| {
            let GtfsStop {
                stop_name,
                stop_lat,
                stop_lon,
                ..
            } = stop;
            let name = stop_name;
            let point = geo::Point::new(stop_lon, stop_lat);
            let poi_type = PointOfInterestType::RailwayStation;

            PointOfInterest {
                id: generate_ulid::<Utc, _>(None, (&name, &point, &poi_type)).unwrap(),
                name,
                point,
                point_of_interest_type: poi_type,
            }
        });

    Ok(pois
        .filter(|poi| poi.name.contains("Railway Station"))
        .collect::<Vec<_>>())
}

pub fn load_huts() -> Result<Vec<PointOfInterest>, anyhow::Error> {
    let data = fs::read(get_project_root()?.join("data/HUTS.gpx"))?;
    let gpx = gpx::read(&*data)?;

    Ok(gpx
        .waypoints
        .into_iter()
        .map(|mut waypoint| {
            waypoint.type_ = Some("HUT".to_string());
            waypoint
        })
        .map(PointOfInterest::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn load_localities() -> Result<Vec<PointOfInterest>, anyhow::Error> {
    let mut reader = shapefile::Reader::from_path(
        get_project_root()?.join("data/vic_localities/vic_localities.shp"),
    )?;

    let pois = reader
        .iter_shapes_and_records()
        .map(|shape_record| -> Result<PointOfInterest, anyhow::Error> {
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

            Ok(PointOfInterest {
                id: generate_ulid::<Utc, _>(
                    None,
                    (
                        &name,
                        polygon.centroid().unwrap(),
                        howitt::models::point_of_interest::PointOfInterestType::Locality,
                    ),
                )
                .unwrap(),
                name,
                point: polygon.centroid().unwrap(),
                point_of_interest_type:
                    howitt::models::point_of_interest::PointOfInterestType::Locality,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(pois)
}

fn ring_to_linestring(ring: &PolygonRing<Point>) -> LineString<f64> {
    geo::LineString::from_iter(ring.points().iter().map(|point| (point.x, point.y)))
}

fn convert_polygon(polygon: GenericPolygon<Point>) -> Polygon<f64> {
    let exterior = polygon
        .rings()
        .iter()
        .find(|ring| matches!(ring, PolygonRing::Outer(_)))
        .unwrap();
    let interiors = polygon
        .rings()
        .iter()
        .filter(|ring| matches!(ring, PolygonRing::Inner(_)));

    Polygon::new(
        ring_to_linestring(exterior),
        interiors.map(ring_to_linestring).collect(),
    )
}
