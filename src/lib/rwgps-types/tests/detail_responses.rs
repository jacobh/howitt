use rwgps_types::{Point, RouteResponse, TripResponse};

#[test]
fn route_detail_accepts_current_activity_and_sparse_course_point_shapes() {
    let response: RouteResponse =
        serde_json::from_str(include_str!("fixtures/route-detail.json")).unwrap();

    assert_eq!(
        response.route.first_point,
        Some(Point {
            lat: 45.1,
            lng: -122.1,
        })
    );
    assert_eq!(response.route.activity_types, ["cycling"]);
    assert_eq!(response.route.course_points[0].note, None);
    assert_eq!(response.route.course_points[0].point_type, None);
    let serialized = serde_json::to_value(response).unwrap();
    assert_eq!(serialized["route"]["first_lat"], 45.1);
    assert_eq!(serialized["route"]["first_lng"], -122.1);
}

#[test]
fn route_detail_accepts_null_or_omitted_bounding_box_coordinate_pairs() {
    let mut fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/route-detail.json")).unwrap();
    fixture["route"]["bounding_box"][0]["lat"] = serde_json::Value::Null;
    fixture["route"]["bounding_box"][0]["lng"] = serde_json::Value::Null;
    fixture["route"]["bounding_box"][1]
        .as_object_mut()
        .unwrap()
        .remove("lat");
    fixture["route"]["bounding_box"][1]
        .as_object_mut()
        .unwrap()
        .remove("lng");

    let response: RouteResponse = serde_json::from_value(fixture).unwrap();

    assert_eq!(response.route.bounding_box, [None, None]);
}

#[test]
fn route_detail_rejects_partial_bounding_box_coordinate_pairs() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/route-detail.json")).unwrap();
    let mut null_latitude = fixture.clone();
    null_latitude["route"]["bounding_box"][0]["lat"] = serde_json::Value::Null;
    let mut omitted_longitude = fixture;
    omitted_longitude["route"]["bounding_box"][1]
        .as_object_mut()
        .unwrap()
        .remove("lng");

    for partial_pair in [null_latitude, omitted_longitude] {
        let error = serde_json::from_value::<RouteResponse>(partial_pair).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("bounding box lat and lng must both be present or absent")
        );
    }
}

#[test]
fn trip_detail_accepts_omitted_last_coordinates() {
    let response: TripResponse =
        serde_json::from_str(include_str!("fixtures/trip-detail.json")).unwrap();

    assert_eq!(
        response.trip.first_point,
        Some(Point {
            lat: 45.1,
            lng: -122.1,
        })
    );
    assert_eq!(response.trip.last_point, None);
    assert_eq!(response.trip.metrics.grade, None);
}

#[test]
fn details_accept_null_first_coordinates() {
    let mut route: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/route-detail.json")).unwrap();
    route["route"]["first_lat"] = serde_json::Value::Null;
    route["route"]["first_lng"] = serde_json::Value::Null;
    let route: RouteResponse = serde_json::from_value(route).unwrap();
    assert_eq!(route.route.first_point, None);

    let mut trip: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/trip-detail.json")).unwrap();
    trip["trip"]["first_lat"] = serde_json::Value::Null;
    trip["trip"]["first_lng"] = serde_json::Value::Null;
    let trip: TripResponse = serde_json::from_value(trip).unwrap();
    assert_eq!(trip.trip.first_point, None);
    let serialized = serde_json::to_value(trip).unwrap();
    assert_eq!(serialized["trip"]["first_lat"], serde_json::Value::Null);
    assert_eq!(serialized["trip"]["first_lng"], serde_json::Value::Null);
}

#[test]
fn details_reject_partial_first_coordinates() {
    let mut route: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/route-detail.json")).unwrap();
    route["route"]["first_lng"] = serde_json::Value::Null;

    let error = serde_json::from_value::<RouteResponse>(route).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("first_lat and first_lng must both be present or absent")
    );
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
