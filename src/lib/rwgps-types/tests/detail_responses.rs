use rwgps_types::{Point, RouteResponse, TripResponse};

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

    assert_eq!(response.trip.last_point, None);
    assert_eq!(response.trip.metrics.grade, None);
}

#[test]
fn trip_detail_accepts_null_last_coordinates() {
    let mut fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/trip-detail.json")).unwrap();
    fixture["trip"]["last_lat"] = serde_json::Value::Null;
    fixture["trip"]["last_lng"] = serde_json::Value::Null;

    let response: TripResponse = serde_json::from_value(fixture).unwrap();

    assert_eq!(response.trip.last_point, None);
    let serialized = serde_json::to_value(response).unwrap();
    assert_eq!(serialized["trip"]["last_lat"], serde_json::Value::Null);
    assert_eq!(serialized["trip"]["last_lng"], serde_json::Value::Null);
}

#[test]
fn trip_detail_accepts_complete_last_coordinates() {
    let mut fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/trip-detail.json")).unwrap();
    fixture["trip"]["last_lat"] = serde_json::json!(45.2);
    fixture["trip"]["last_lng"] = serde_json::json!(-122.0);

    let response: TripResponse = serde_json::from_value(fixture).unwrap();

    assert_eq!(
        response.trip.last_point,
        Some(Point {
            lat: 45.2,
            lng: -122.0,
        })
    );
    let serialized = serde_json::to_value(response).unwrap();
    assert_eq!(serialized["trip"]["last_lat"], 45.2);
    assert_eq!(serialized["trip"]["last_lng"], -122.0);
}

#[test]
fn trip_detail_rejects_partial_last_coordinates() {
    let mut fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/trip-detail.json")).unwrap();
    fixture["trip"]["last_lat"] = serde_json::json!(45.2);

    let error = serde_json::from_value::<TripResponse>(fixture).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("last_lat and last_lng must both be present or absent")
    );
}
