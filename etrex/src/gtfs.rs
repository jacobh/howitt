use std::io::{Read, Seek};

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Data parse failed")]
pub enum GtfsParseError {
    Zip(#[from] zip::result::ZipError),
    Io(#[from] std::io::Error),
    Csv(#[from] csv::Error),
}

#[derive(Debug)]
pub struct GtfsZip {
    pub stops: Vec<GtfsStop>,
}
impl GtfsZip {
    pub fn parse(data: impl Read + Seek) -> Result<GtfsZip, GtfsParseError> {
        let mut zip = zip::ZipArchive::new(data)?;

        let stops = csv::Reader::from_reader(zip.by_name("stops.txt")?)
            .into_deserialize()
            .collect::<Result<Vec<GtfsStop>, _>>()?;

            Ok(GtfsZip { stops })
    }
}

#[derive(Debug, Deserialize)]
pub struct GtfsStop {
    pub stop_id: String,
    pub stop_name: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
}
