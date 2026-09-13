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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use zip::{ZipWriter, write::SimpleFileOptions};

    #[test]
    fn parses_stops_from_a_gtfs_zip() {
        let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
        archive
            .start_file("stops.txt", SimpleFileOptions::default())
            .unwrap();
        archive
            .write_all(
                b"stop_id,stop_name,stop_lat,stop_lon\n\
                  syd-opera,\"Sydney Opera House, East\",-33.8568,151.2153\n\
                  perth,Perth Station,-31.9527,115.8605\n",
            )
            .unwrap();
        let data = archive.finish().unwrap().into_inner();

        let gtfs = GtfsZip::parse(Cursor::new(data)).unwrap();

        assert_eq!(gtfs.stops.len(), 2);
        assert_eq!(gtfs.stops[0].stop_id, "syd-opera");
        assert_eq!(gtfs.stops[0].stop_name, "Sydney Opera House, East");
        assert_eq!(gtfs.stops[0].stop_lat, -33.8568);
        assert_eq!(gtfs.stops[0].stop_lon, 151.2153);
        assert_eq!(gtfs.stops[1].stop_id, "perth");
        assert_eq!(gtfs.stops[1].stop_name, "Perth Station");
        assert_eq!(gtfs.stops[1].stop_lat, -31.9527);
        assert_eq!(gtfs.stops[1].stop_lon, 115.8605);
    }
}
