use rwgps_types::{RouteResponse, TripResponse};

#[test]
fn route_detail_accepts_current_activity_and_sparse_course_point_shapes() {
    let response: RouteResponse =
        serde_json::from_str(include_str!("fixtures/route-detail.json")).unwrap();

    assert_eq!(response.route.activity_types, ["cycling"]);
    assert_eq!(response.route.course_points[0].note, None);
    assert_eq!(response.route.course_points[0].point_type, None);
}

#[test]
fn trip_detail_accepts_omitted_last_coordinates() {
    let response: TripResponse =
        serde_json::from_str(include_str!("fixtures/trip-detail.json")).unwrap();

    assert_eq!(response.trip.last_lat, None);
    assert_eq!(response.trip.last_lng, None);
    assert_eq!(response.trip.metrics.grade, None);
}

#[test]
fn trip_detail_accepts_null_last_coordinates() {
    let mut fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/trip-detail.json")).unwrap();
    fixture["trip"]["last_lat"] = serde_json::Value::Null;
    fixture["trip"]["last_lng"] = serde_json::Value::Null;

    let response: TripResponse = serde_json::from_value(fixture).unwrap();

    assert_eq!(response.trip.last_lat, None);
    assert_eq!(response.trip.last_lng, None);
}
