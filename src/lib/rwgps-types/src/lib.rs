use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_default_from_empty_object;
use serde_json::Value;

pub mod client;
pub mod config;
pub mod credentials;
pub mod webhook;

#[derive(Deserialize, Debug)]
pub struct AuthenticatedUserDetailResponse {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub auth_token: String,
    pub id: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub results: Vec<T>,
    pub results_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TripSummary {
    pub id: usize,
    pub group_membership_id: usize,
    pub route_id: Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub gear_id: Option<usize>,
    pub departed_at: chrono::DateTime<chrono::Utc>,
    pub duration: i64,
    pub distance: f64,
    pub elevation_gain: Option<f64>,
    pub elevation_loss: Option<f64>,
    pub visibility: i64,
    pub description: Option<String>,
    pub is_gps: bool,
    pub name: String,
    pub max_hr: Option<i64>,
    pub min_hr: Option<i64>,
    pub avg_hr: Option<i64>,
    pub max_cad: Value,
    pub min_cad: Value,
    pub avg_cad: Value,
    pub avg_speed: f64,
    pub max_speed: f64,
    pub moving_time: i64,
    pub processed: bool,
    pub avg_watts: Option<f64>,
    pub max_watts: Value,
    pub min_watts: Value,
    pub is_stationary: bool,
    pub calories: Option<i64>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub time_zone: String,
    pub first_lng: f64,
    pub first_lat: f64,
    pub last_lng: f64,
    pub last_lat: f64,
    pub user_id: usize,
    pub deleted_at: Value,
    pub sw_lng: f64,
    pub sw_lat: f64,
    pub ne_lng: f64,
    pub ne_lat: f64,
    pub track_id: String,
    pub postal_code: Option<String>,
    pub locality: Option<String>,
    pub administrative_area: Option<String>,
    pub country_code: String,
    pub source_type: Option<String>,
    pub likes_count: usize,
    pub track_type: String,
    pub terrain: String,
    pub difficulty: String,
    pub activity_type_id: usize,
    pub activity_category_id: usize,
    pub highlighted_photo_id: usize,
    pub highlighted_photo_checksum: Option<String>,
    pub utc_offset: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSummary {
    pub administrative_area: Option<String>,
    pub archived_at: Value,
    pub best_for_id: Option<usize>,
    pub country_code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Value,
    pub description: Option<String>,
    pub difficulty: String,
    pub distance: Option<f64>,
    pub elevation_gain: Option<f64>,
    pub elevation_loss: Option<f64>,
    pub first_lat: f64,
    pub first_lng: f64,
    pub group_membership_id: usize,
    pub has_course_points: bool,
    pub highlighted_photo_checksum: Value,
    pub highlighted_photo_id: usize,
    pub id: usize,
    pub is_trip: bool,
    pub last_lat: f64,
    pub last_lng: f64,
    pub likes_count: usize,
    pub locality: Option<String>,
    pub name: String,
    pub ne_lat: f64,
    pub ne_lng: f64,
    pub pavement_type_id: Option<usize>,
    pub planner_options: Option<i64>,
    pub postal_code: Option<String>,
    pub sw_lat: f64,
    pub sw_lng: f64,
    pub terrain: String,
    pub track_id: String,
    pub track_type: String,
    pub unpaved_pct: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: usize,
    pub visibility: i64,
}

// route detail

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub route: Route,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub id: usize,
    pub highlighted_photo_id: usize,
    pub highlighted_photo_checksum: Value,
    pub distance: Option<f64>,
    pub elevation_gain: Option<f64>,
    pub elevation_loss: Option<f64>,
    pub track_id: String,
    pub user_id: usize,
    pub pavement_type: Value,
    pub pavement_type_id: Value,
    pub recreation_type_ids: Vec<Value>,
    pub visibility: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: Option<String>,
    pub first_lng: f64,
    pub first_lat: f64,
    pub last_lat: f64,
    pub last_lng: f64,
    pub bounding_box: Vec<Point>,
    pub locality: Option<String>,
    pub postal_code: Option<String>,
    pub administrative_area: Option<String>,
    pub country_code: String,
    pub privacy_code: Value,
    pub user: User,
    pub has_course_points: bool,
    pub tag_names: Vec<Value>,
    pub track_type: String,
    pub terrain: String,
    pub difficulty: String,
    pub unpaved_pct: f64,
    pub surface: String,
    pub nav_enabled: bool,
    pub rememberable: bool,
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub metrics: Option<Metrics>,
    pub photos: Vec<Photo>,
    pub segment_matches: Option<Vec<SegmentMatch>>,
    pub track_points: Vec<TrackPoint>,
    pub course_points: Vec<CoursePoint>,
    pub points_of_interest: Vec<Value>,
}
impl Route {
    fn url(&self) -> String {
        format!("https://ridewithgps.com/routes/{}", self.id)
    }
}

impl From<Route> for gpx::Gpx {
    fn from(value: Route) -> Self {
        gpx::Gpx {
            version: gpx::GpxVersion::Gpx11,
            creator: Some(value.user.name.clone()),
            metadata: None,
            waypoints: vec![],
            tracks: vec![],
            routes: vec![gpx::Route::from(value)],
        }
    }
}

impl From<Route> for gpx::Route {
    fn from(value: Route) -> Self {
        gpx::Route {
            name: Some(value.name.clone()),
            comment: None,
            description: value.description.clone(),
            source: Some(value.url()),
            links: vec![gpx::Link {
                href: value.url(),
                text: Some(value.name.clone()),
                type_: None,
            }],
            number: None,
            type_: None,
            points: value
                .track_points
                .into_iter()
                .map(gpx::Waypoint::try_from)
                .filter_map(Result::ok)
                .collect(),
        }
    }
}

impl From<Route> for geo::LineString<f64> {
    fn from(value: Route) -> Self {
        geo::LineString::from_iter(
            value
                .track_points
                .into_iter()
                .map(geo::Point::try_from)
                .filter_map(Result::ok),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TripResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub trip: Trip,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trip {
    pub id: usize,
    pub highlighted_photo_id: usize,
    pub highlighted_photo_checksum: Value,
    pub distance: f64,
    pub elevation_gain: Option<f64>,
    pub elevation_loss: Option<f64>,
    pub track_id: String,
    pub user_id: usize,
    pub visibility: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub departed_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: Option<String>,
    pub first_lng: f64,
    pub first_lat: f64,
    pub last_lat: f64,
    pub last_lng: f64,
    pub bounding_box: Vec<Point>,
    pub locality: Value,
    pub postal_code: Value,
    pub administrative_area: Value,
    pub country_code: Value,
    pub is_stationary: bool,
    pub privacy_code: Value,
    pub user: User,
    pub gear: Option<Gear>,
    pub tag_names: Vec<Value>,
    pub track_type: String,
    pub terrain: String,
    pub difficulty: String,
    pub metrics: Metrics,
    pub live_logging: bool,
    pub live_log: Value,
    pub rememberable: bool,
    pub photos: Vec<Value>,
    pub track_points: Vec<TrackPoint>,
    pub course_points: Vec<Value>,
    pub points_of_interest: Vec<Value>,
}

impl From<Trip> for geo::LineString<f64> {
    fn from(value: Trip) -> Self {
        geo::LineString::from_iter(
            value
                .track_points
                .into_iter()
                .map(geo::Point::try_from)
                .filter_map(Result::ok),
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Value,
    pub interests: Value,
    pub locality: Option<String>,
    pub administrative_area: Option<String>,
    pub account_level: i64,
    pub total_trip_distance: f64,
    pub total_trip_duration: i64,
    pub total_trip_elevation_gain: Option<f64>,
    pub name: String,
    pub highlighted_photo_id: usize,
    pub highlighted_photo_checksum: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    pub id: Option<usize>,
    pub parent_id: Option<usize>,
    pub parent_type: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ele: Option<Elevation>,
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub grade: Option<Grade>,
    pub distance: Option<f64>,
    #[serde(rename = "startElevation")]
    pub start_elevation: Option<f64>,
    #[serde(rename = "endElevation")]
    pub end_elevation: Option<f64>,
    #[serde(rename = "numPoints")]
    pub num_points: Option<i64>,
    pub ele_gain: Option<f64>,
    pub ele_loss: Option<f64>,
    pub v: Option<i64>,
    #[serde(default)]
    pub hills: Vec<Hill>,
    pub watts: Option<Value>,
    pub cad: Option<Value>,
    pub hr: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elevation {
    pub max: f64,
    pub min: f64,
    #[serde(rename = "_min")]
    pub min2: f64,
    #[serde(rename = "_max")]
    pub max2: f64,
    pub min_i: Option<f64>,
    pub max_i: Option<f64>,
    #[serde(rename = "_avg")]
    pub avg: f64,
    #[serde(rename = "avg")]
    pub avg2: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grade {
    pub max: f64,
    pub min: f64,
    #[serde(rename = "_min")]
    pub min2: f64,
    #[serde(rename = "_max")]
    pub max2: f64,
    pub max_i: Option<f64>,
    pub min_i: Option<f64>,
    #[serde(rename = "_avg")]
    pub avg: f64,
    #[serde(rename = "avg")]
    pub avg2: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hill {
    pub first_i: f64,
    pub last_i: f64,
    pub ele_gain: f64,
    pub ele_loss: f64,
    pub distance: f64,
    pub avg_grade: f64,
    pub is_climb: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentMatch {
    pub id: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub mongo_id: Option<String>,
    pub user_id: usize,
    pub segment_id: usize,
    pub parent_type: String,
    pub parent_id: usize,
    pub final_time: Value,
    pub visibility: i64,
    pub start_index: i64,
    pub end_index: i64,
    pub duration: Value,
    pub moving_time: Value,
    pub ascent_time: Value,
    pub personal_record: Value,
    pub vam: Value,
    pub started_at: Value,
    pub distance: f64,
    pub avg_speed: Value,
    pub rank: Value,
    pub segment: Segment,
    pub metrics: Metrics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub title: String,
    pub slug: String,
    pub to_param: String,
}

#[serde_with::serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct TrackPoint {
    #[serde(rename = "R")]
    pub r: Option<i64>,
    #[serde(rename = "S")]
    pub s: Option<i64>,
    #[serde(rename = "d")]
    pub distance: Option<f64>,
    #[serde(rename = "e")]
    pub elevation: Option<f64>,
    #[serde(rename = "x")]
    pub lng: Option<f64>,
    #[serde(rename = "y")]
    pub lat: Option<f64>,
    #[serde(rename = "t")]
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac<f64>>")]
    pub datetime: Option<DateTime<Utc>>,
    pub color: Option<i64>,
    pub options: Option<i64>,
}

impl TryFrom<TrackPoint> for geo::Point<f64> {
    type Error = ();

    fn try_from(value: TrackPoint) -> Result<Self, Self::Error> {
        match (value.lng, value.lat) {
            (Some(lng), Some(lat)) => Ok(geo::Point::new(lng, lat)),
            _ => Err(()),
        }
    }
}

impl TryFrom<TrackPoint> for gpx::Waypoint {
    type Error = ();

    fn try_from(value: TrackPoint) -> Result<Self, Self::Error> {
        geo::Point::try_from(value).map(gpx::Waypoint::new)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoursePoint {
    #[serde(rename = "d")]
    pub distance: Option<f64>,
    pub i: i64,
    #[serde(rename = "n")]
    pub note: String,
    #[serde(rename = "t")]
    pub point_type: String,
    #[serde(rename = "x")]
    pub lng: f64,
    #[serde(rename = "y")]
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gear {
    pub id: usize,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photo {
    pub id: usize,
    pub group_membership_id: usize,
    pub caption: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub position: Option<i64>,
    pub visibility: i64,
    pub lat: Value,
    pub lng: Value,
    pub published: Option<bool>,
    pub captured_at: Option<chrono::DateTime<chrono::Utc>>,
    pub user_id: usize,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub width: usize,
    pub height: usize,
    pub optional_uuid: Value,
    pub checksum: String,
}
