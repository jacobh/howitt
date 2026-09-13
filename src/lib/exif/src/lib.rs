use chrono::{DateTime, Local, TimeZone, Utc};
use nom_exif::{ExifDateTime, ExifTag, GPSInfo, MediaParser, MediaSource};
use std::io::Cursor;

pub fn gps_info_to_point(gps_info: GPSInfo) -> geo::Point<f64> {
    let lat =
        gps_info.latitude.to_decimal_degrees().unwrap_or(f64::NAN) * gps_info.latitude_ref.sign();
    let lon =
        gps_info.longitude.to_decimal_degrees().unwrap_or(f64::NAN) * gps_info.longitude_ref.sign();

    geo::Point::new(lon, lat)
}

#[derive(Debug)]
pub struct ParsedExifData {
    pub captured_at: Option<DateTime<Utc>>,
    pub point: Option<geo::Point<f64>>,
}

pub fn parse_exif(bytes: &[u8]) -> ParsedExifData {
    let mut captured_at = None;
    let mut point = None;

    let mut parser = MediaParser::new();
    if let Ok(ms) = MediaSource::seekable(Cursor::new(bytes)) {
        if let Ok(exif) = parser.parse_exif(ms) {
            let exif = nom_exif::Exif::from(exif);

            // Get captured_at from DateTimeOriginal or CreateDate
            if let Some(entry) = exif
                .get(ExifTag::DateTimeOriginal)
                .or_else(|| exif.get(ExifTag::CreateDate))
            {
                captured_at = entry.as_datetime().and_then(|dt| match dt {
                    ExifDateTime::Aware(dt) => Some(dt.with_timezone(&Utc)),
                    ExifDateTime::Naive(dt) => Local
                        .from_local_datetime(&dt)
                        .single()
                        .map(|dt| dt.with_timezone(&Utc)),
                });
            }

            // Get GPS coordinates
            if let Some(gps_info) = exif.gps_info() {
                point = Some(gps_info_to_point(gps_info.clone()));
            }
        }
    }

    ParsedExifData { captured_at, point }
}

#[cfg(test)]
mod tests {
    use nom_exif::{LatLng, LatRef, LonRef, URational};

    use super::*;

    #[test]
    fn test_gps_info_to_point() {
        // Sydney Opera House, Australia (-33.8568, 151.2153)
        let sydney = GPSInfo {
            latitude_ref: LatRef::South,
            latitude: LatLng::new(
                URational::new(33, 1),
                URational::new(51, 1),
                URational::new(24, 1),
            ),
            longitude_ref: LonRef::East,
            longitude: LatLng::new(
                URational::new(151, 1),
                URational::new(12, 1),
                URational::new(55, 1),
            ),
            ..Default::default()
        };
        let sydney_point = gps_info_to_point(sydney);
        assert!((sydney_point.x() - 151.2153).abs() < 0.01);
        assert!((sydney_point.y() - (-33.8568)).abs() < 0.01);

        // CN Tower, Toronto, Canada (43.6426, -79.3871)
        let toronto = GPSInfo {
            latitude_ref: LatRef::North,
            latitude: LatLng::new(
                URational::new(43, 1),
                URational::new(38, 1),
                URational::new(33, 1),
            ),
            longitude_ref: LonRef::West,
            longitude: LatLng::new(
                URational::new(79, 1),
                URational::new(23, 1),
                URational::new(14, 1),
            ),
            ..Default::default()
        };
        let toronto_point = gps_info_to_point(toronto);
        assert!((toronto_point.x() - (-79.3871)).abs() < 0.01);
        assert!((toronto_point.y() - 43.6426).abs() < 0.01);

        // Christ the Redeemer, Rio de Janeiro, Brazil (-22.9519, -43.2105)
        let rio = GPSInfo {
            latitude_ref: LatRef::South,
            latitude: LatLng::new(
                URational::new(22, 1),
                URational::new(57, 1),
                URational::new(7, 1),
            ),
            longitude_ref: LonRef::West,
            longitude: LatLng::new(
                URational::new(43, 1),
                URational::new(12, 1),
                URational::new(38, 1),
            ),
            ..Default::default()
        };
        let rio_point = gps_info_to_point(rio);
        assert!((rio_point.x() - (-43.2105)).abs() < 0.01);
        assert!((rio_point.y() - (-22.9519)).abs() < 0.01);

        // Eiffel Tower, Paris, France (48.8584, 2.2945)
        let paris = GPSInfo {
            latitude_ref: LatRef::North,
            latitude: LatLng::new(
                URational::new(48, 1),
                URational::new(51, 1),
                URational::new(30, 1),
            ),
            longitude_ref: LonRef::East,
            longitude: LatLng::new(
                URational::new(2, 1),
                URational::new(17, 1),
                URational::new(40, 1),
            ),
            ..Default::default()
        };
        let paris_point = gps_info_to_point(paris);
        assert!((paris_point.x() - 2.2945).abs() < 0.01);
        assert!((paris_point.y() - 48.8584).abs() < 0.01);

        // Tokyo Tower, Japan (35.6586, 139.7454)
        let tokyo = GPSInfo {
            latitude_ref: LatRef::North,
            latitude: LatLng::new(
                URational::new(35, 1),
                URational::new(39, 1),
                URational::new(31, 1),
            ),
            longitude_ref: LonRef::East,
            longitude: LatLng::new(
                URational::new(139, 1),
                URational::new(44, 1),
                URational::new(43, 1),
            ),
            ..Default::default()
        };
        let tokyo_point = gps_info_to_point(tokyo);
        assert!((tokyo_point.x() - 139.7454).abs() < 0.01);
        assert!((tokyo_point.y() - 35.6586).abs() < 0.01);
    }
}
