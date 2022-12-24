use std::io::{Read, Seek};

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Data parse failed")]
pub enum GtfsParseError {
    Zip(#[from] zip::result::ZipError),
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct GtfsZip {
    pub stops: Vec<GtfsStop>,
}
impl GtfsZip {
    pub fn parse(data: impl Read + Seek) -> Result<GtfsZip, GtfsParseError> {
        let mut zip = zip::ZipArchive::new(data)?;
        // dbg!(zip.file_names().collect::<Vec<_>>());

        let stops_bytes = {
            let mut buf = Vec::new();
            zip.by_name("stops.txt")?.read_to_end(&mut buf)?;
            buf
        };

        let stops_string = String::from_utf8(stops_bytes).unwrap();
        let stops = stops_string
            .lines()
            .skip(1)
            .map(|line| {
                let cols: Vec<String> = line
                    .to_owned()
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap()
                    .split("\",\"")
                    .map(|col| col.to_string())
                    .collect();

                GtfsStop {
                    id: (&cols[0]).to_owned(),
                    name: (&cols[1]).to_owned(),
                    point: geo::Point::new(cols[3].parse().unwrap(), cols[2].parse().unwrap()),
                }
            })
            .collect();

        // dbg!(stops_data);
        Ok(GtfsZip { stops })
    }
}

#[derive(Debug)]
pub struct GtfsStop {
    pub id: String,
    pub name: String,
    pub point: geo::Point<f64>,
}
