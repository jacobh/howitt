use std::io::Cursor;

use apalis::prelude::*;
use axum::{body::Bytes, extract::State, http::StatusCode, Json};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use chrono::{DateTime, Utc};
use howitt::{
    jobs::{
        media::{MediaJob, ProcessMedia},
        Job,
    },
    models::media::{Media, MediaId, MediaRelationId},
    repos::Repo,
    services::{
        media::{generate_media_key, GenerateMediaKeyParams},
        user::auth::Login,
    },
};
use howitt_client_types::{BucketClient, ObjectParams};
use nom_exif::{ExifIter, ExifTag, GPSInfo, MediaParser, MediaSource};
use serde_json::json;

use crate::app_state::AppState;

#[derive(TryFromMultipart)]
pub struct UploadMediaRequest {
    #[form_data(limit = "unlimited")]
    pub file: FieldData<Bytes>,
    pub name: String,
    pub relation_ids: Option<String>, // JSON array of relation IDs
}

fn parse_relation_ids(
    relation_ids: Option<String>,
) -> Result<Vec<MediaRelationId>, serde_json::Error> {
    relation_ids
        .map(|ids| serde_json::from_str(&ids))
        .unwrap_or(Ok(vec![]))
}

fn gps_info_to_point(gps_info: GPSInfo) -> geo::Point<f64> {
    let lat = gps_info.latitude.0.as_float()
        + gps_info.latitude.1.as_float() / 60.0
        + gps_info.latitude.2.as_float() / 3600.0;
    let lon = gps_info.longitude.0.as_float()
        + gps_info.longitude.1.as_float() / 60.0
        + gps_info.longitude.2.as_float() / 3600.0;

    let lat = if gps_info.latitude_ref == 'S' {
        -lat
    } else {
        lat
    };
    let lon = if gps_info.longitude_ref == 'W' {
        -lon
    } else {
        lon
    };

    geo::Point::new(lon, lat)
}

#[derive(Debug)]
struct ExifData {
    captured_at: Option<DateTime<Utc>>,
    point: Option<geo::Point<f64>>,
}

fn parse_exif(bytes: &[u8]) -> ExifData {
    let mut captured_at = None;
    let mut point = None;

    let mut parser = MediaParser::new();
    if let Ok(ms) = MediaSource::seekable(Cursor::new(bytes)) {
        if ms.has_exif() {
            if let Ok(exif) = parser.parse::<_, _, ExifIter>(ms) {
                let exif = nom_exif::Exif::from(exif);

                // Get captured_at from DateTimeOriginal or CreateDate
                if let Some(entry) = exif
                    .get(ExifTag::DateTimeOriginal)
                    .or_else(|| exif.get(ExifTag::CreateDate))
                {
                    if let Some(dt) = entry.as_time() {
                        captured_at = Some(dt.into());
                    }
                }

                // Get GPS coordinates
                if let Ok(Some(gps_info)) = exif.get_gps_info() {
                    point = Some(gps_info_to_point(gps_info));
                }
            }
        }
    }

    ExifData { captured_at, point }
}

pub async fn upload_media_handler(
    State(AppState {
        bucket_client,
        media_repo,
        job_storage,
        ..
    }): State<AppState>,
    login: Login,
    TypedMultipart(upload): TypedMultipart<UploadMediaRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Generate unique ID and S3 key
    let media_id = MediaId::new();
    let key = generate_media_key(GenerateMediaKeyParams {
        media_id,
        user_id: login.session.user_id,
        name: upload.name,
    });

    // Get the file contents
    let bytes = upload.file.contents;

    let kind = infer::get(&bytes).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Could not determine file type"})),
        )
    })?;

    let ExifData { captured_at, point } = parse_exif(&bytes);

    let params = ObjectParams {
        content_type: Some(kind.mime_type().to_string()),
    };

    // Upload to S3
    bucket_client
        .put_object(&key, bytes.into(), params)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to upload to S3: {}", e)})),
            )
        })?;

    let relation_ids = parse_relation_ids(upload.relation_ids).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to parse relation ids: {}", e)})),
        )
    })?;

    // Create media record
    let media = Media {
        id: media_id.clone(),
        created_at: chrono::Utc::now(),
        user_id: login.session.user_id,
        path: key,
        relation_ids,
        point,
        captured_at,
    };

    // Save to database
    media_repo.put(media).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to save to database: {}", e)})),
        )
    })?;

    job_storage
        .lock()
        .await
        .push(Job::from(MediaJob::from(ProcessMedia {
            media_id: media_id.clone(),
        })))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to enqueue job: {}", e)})),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "media_id": media_id.to_string()
        })),
    ))
}

#[cfg(test)]
mod tests {
    use nom_exif::LatLng;

    use super::*;

    #[test]
    fn test_gps_info_to_point() {
        // Sydney Opera House, Australia (-33.8568, 151.2153)
        let sydney = GPSInfo {
            latitude_ref: 'S',
            latitude: LatLng((33, 1).into(), (51, 1).into(), (24, 1).into()),
            longitude_ref: 'E',
            longitude: LatLng((151, 1).into(), (12, 1).into(), (55, 1).into()),
            altitude_ref: 0,
            altitude: (0, 1).into(),
            speed_ref: None,
            speed: None,
        };
        let sydney_point = gps_info_to_point(sydney);
        assert!((sydney_point.x() - 151.2153).abs() < 0.01);
        assert!((sydney_point.y() - (-33.8568)).abs() < 0.01);

        // CN Tower, Toronto, Canada (43.6426, -79.3871)
        let toronto = GPSInfo {
            latitude_ref: 'N',
            latitude: LatLng((43, 1).into(), (38, 1).into(), (33, 1).into()),
            longitude_ref: 'W',
            longitude: LatLng((79, 1).into(), (23, 1).into(), (14, 1).into()),
            altitude_ref: 0,
            altitude: (0, 1).into(),
            speed_ref: None,
            speed: None,
        };
        let toronto_point = gps_info_to_point(toronto);
        assert!((toronto_point.x() - (-79.3871)).abs() < 0.01);
        assert!((toronto_point.y() - 43.6426).abs() < 0.01);

        // Christ the Redeemer, Rio de Janeiro, Brazil (-22.9519, -43.2105)
        let rio = GPSInfo {
            latitude_ref: 'S',
            latitude: LatLng((22, 1).into(), (57, 1).into(), (7, 1).into()),
            longitude_ref: 'W',
            longitude: LatLng((43, 1).into(), (12, 1).into(), (38, 1).into()),
            altitude_ref: 0,
            altitude: (0, 1).into(),
            speed_ref: None,
            speed: None,
        };
        let rio_point = gps_info_to_point(rio);
        assert!((rio_point.x() - (-43.2105)).abs() < 0.01);
        assert!((rio_point.y() - (-22.9519)).abs() < 0.01);

        // Eiffel Tower, Paris, France (48.8584, 2.2945)
        let paris = GPSInfo {
            latitude_ref: 'N',
            latitude: LatLng((48, 1).into(), (51, 1).into(), (30, 1).into()),
            longitude_ref: 'E',
            longitude: LatLng((2, 1).into(), (17, 1).into(), (40, 1).into()),
            altitude_ref: 0,
            altitude: (0, 1).into(),
            speed_ref: None,
            speed: None,
        };
        let paris_point = gps_info_to_point(paris);
        assert!((paris_point.x() - 2.2945).abs() < 0.01);
        assert!((paris_point.y() - 48.8584).abs() < 0.01);

        // Tokyo Tower, Japan (35.6586, 139.7454)
        let tokyo = GPSInfo {
            latitude_ref: 'N',
            latitude: LatLng((35, 1).into(), (39, 1).into(), (31, 1).into()),
            longitude_ref: 'E',
            longitude: LatLng((139, 1).into(), (44, 1).into(), (43, 1).into()),
            altitude_ref: 0,
            altitude: (0, 1).into(),
            speed_ref: None,
            speed: None,
        };
        let tokyo_point = gps_info_to_point(tokyo);
        assert!((tokyo_point.x() - 139.7454).abs() < 0.01);
        assert!((tokyo_point.y() - 35.6586).abs() < 0.01);
    }
}
