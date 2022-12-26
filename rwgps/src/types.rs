use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub results_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSummary {
    pub administrative_area: String,
    pub archived_at: Value,
    pub best_for_id: Option<i64>,
    pub country_code: String,
    pub created_at: String,
    pub deleted_at: Value,
    pub description: Option<String>,
    pub difficulty: String,
    pub distance: Option<f64>,
    pub elevation_gain: f64,
    pub elevation_loss: f64,
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
    pub likes_count: i64,
    pub locality: String,
    pub name: String,
    pub ne_lat: f64,
    pub ne_lng: f64,
    pub pavement_type_id: Option<i64>,
    pub planner_options: Option<i64>,
    pub postal_code: Option<String>,
    pub sw_lat: f64,
    pub sw_lng: f64,
    pub terrain: String,
    pub track_id: String,
    pub track_type: String,
    pub unpaved_pct: i64,
    pub updated_at: String,
    pub user_id: usize,
    pub visibility: i64,
}

// route detail

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub route: Route,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub id: usize,
    pub highlighted_photo_id: usize,
    pub highlighted_photo_checksum: Value,
    pub distance: f64,
    pub elevation_gain: f64,
    pub elevation_loss: f64,
    pub track_id: String,
    pub user_id: usize,
    pub pavement_type: Value,
    pub pavement_type_id: Value,
    pub recreation_type_ids: Vec<Value>,
    pub visibility: i64,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub description: String,
    pub first_lng: f64,
    pub first_lat: f64,
    pub last_lat: f64,
    pub last_lng: f64,
    pub bounding_box: Vec<Point>,
    pub locality: String,
    pub postal_code: String,
    pub administrative_area: String,
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
    pub metrics: Metrics,
    pub photos: Vec<Value>,
    pub segment_matches: Vec<SegmentMatch>,
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
            description: Some(value.description.clone()),
            source: Some(value.url()),
            links: vec![gpx::Link { href: value.url(), text: Some(value.name.clone()), _type: None }],
            number: None,
            _type: None,
            points: value
                .track_points
                .into_iter()
                .map(gpx::Waypoint::from)
                .collect(),
        }
    }
}

impl From<Route> for geo::LineString<f64> {
    fn from(value: Route) -> Self {
        geo::LineString::from_iter(value.track_points.into_iter().map(geo::Point::from))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub created_at: String,
    pub description: Value,
    pub interests: Value,
    pub locality: String,
    pub administrative_area: String,
    pub account_level: i64,
    pub total_trip_distance: f64,
    pub total_trip_duration: i64,
    pub total_trip_elevation_gain: f64,
    pub name: String,
    pub highlighted_photo_id: usize,
    pub highlighted_photo_checksum: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    pub id: usize,
    pub parent_id: usize,
    pub parent_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub ele: Elevation,
    pub grade: Grade,
    pub distance: f64,
    #[serde(rename = "startElevation")]
    pub start_elevation: f64,
    #[serde(rename = "endElevation")]
    pub end_elevation: f64,
    #[serde(rename = "numPoints")]
    pub num_points: i64,
    pub ele_gain: f64,
    pub ele_loss: f64,
    pub v: i64,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentMatch {
    pub id: usize,
    pub created_at: String,
    pub updated_at: String,
    pub mongo_id: String,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackPoint {
    #[serde(rename = "R")]
    pub r: i64,
    #[serde(rename = "S")]
    pub s: i64,
    #[serde(rename = "d")]
    pub distance: f64,
    #[serde(rename = "e")]
    pub elevation: f64,
    #[serde(rename = "x")]
    pub lng: f64,
    #[serde(rename = "y")]
    pub lat: f64,
    pub color: Option<i64>,
    pub options: Option<i64>,
}

impl From<TrackPoint> for geo::Point<f64> {
    fn from(value: TrackPoint) -> Self {
        geo::Point::new(value.lng, value.lat)
    }
}

impl From<TrackPoint> for gpx::Waypoint {
    fn from(value: TrackPoint) -> Self {
        gpx::Waypoint::new(geo::Point::from(value))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoursePoint {
    #[serde(rename = "d")]
    pub distance: f64,
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
