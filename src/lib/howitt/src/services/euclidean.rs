use geo::{
    algorithm::line_measures::metric_spaces::Geodesic, Bearing, Destination, Distance, Euclidean,
    Point,
};

pub fn geo_to_euclidean(points: &[Point<f64>]) -> Vec<Point<f64>> {
    if points.is_empty() {
        return Vec::new();
    }
    let origin = points[0];
    points
        .iter()
        .map(|p| {
            let distance = Geodesic::distance(origin, *p);
            let bearing = Geodesic::bearing(origin, *p);
            let bearing_radians = bearing.to_radians();
            let x = distance * bearing_radians.sin();
            let y = distance * bearing_radians.cos();
            Point::new(x, y)
        })
        .collect()
}

pub fn euclidean_to_geo(origin: Point<f64>, points: &[Point<f64>]) -> Vec<Point<f64>> {
    points
        .iter()
        .map(|p| {
            // Get distance between origin and point using Euclidean distance
            let distance = Euclidean::distance(Point::new(0.0, 0.0), *p);

            // Calculate bearing by getting angle between points
            let bearing = (90.0 - p.y().atan2(p.x()).to_degrees()).rem_euclid(360.0);

            // Use geodesic destination to get final geo point
            Geodesic::destination(origin, bearing, distance)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_conversion_roundtrip() {
        let geo_points_input = vec![
            Point::new(145.691096, -37.753237),
            Point::new(145.814544, -37.767714),
            Point::new(145.92525, -37.827911),
            Point::new(145.915082, -37.773948),
        ];

        let euclidean_points = geo_to_euclidean(&geo_points_input);
        let origin_point = geo_points_input[0];
        let recovered_geo_points = euclidean_to_geo(origin_point, &euclidean_points);

        // Test each point with appropriate epsilon due to floating-point arithmetic
        const EPSILON: f64 = 1e-6;
        for (original, recovered) in geo_points_input.iter().zip(recovered_geo_points.iter()) {
            assert!(
                (original.x() - recovered.x()).abs() < EPSILON,
                "Longitude mismatch. Original: {}, Recovered: {}",
                original.x(),
                recovered.x()
            );
            assert!(
                (original.y() - recovered.y()).abs() < EPSILON,
                "Latitude mismatch. Original: {}, Recovered: {}",
                original.y(),
                recovered.y()
            );
        }
    }

    #[test]
    fn test_empty_points() {
        let empty_geo_points: Vec<Point<f64>> = vec![];
        let euclidean_result = geo_to_euclidean(&empty_geo_points);
        assert!(
            euclidean_result.is_empty(),
            "Should return empty vector for empty input"
        );
    }

    #[test]
    fn test_single_point() {
        let geo_points = vec![Point::new(145.0, -37.0)];
        let euclidean_points = geo_to_euclidean(&geo_points);

        assert_eq!(euclidean_points.len(), 1);
        assert_eq!(
            euclidean_points[0],
            Point::new(0.0, 0.0),
            "Single point should convert to origin in euclidean space"
        );
    }

    #[test]
    fn test_two_points_distance() {
        let geo_points = vec![Point::new(145.0, -37.0), Point::new(145.1, -37.0)];

        let euclidean_points = geo_to_euclidean(&geo_points);
        assert_eq!(euclidean_points.len(), 2);

        // First point should be at origin
        assert!(euclidean_points[0].x().abs() < 1e-10);
        assert!(euclidean_points[0].y().abs() < 1e-10);

        // Second point should be east of origin (positive x)
        assert!(euclidean_points[1].x() > 0.0);
    }

    #[test]
    fn test_euclidean_to_geo_origin() {
        let origin = Point::new(145.0, -37.0);
        let euclidean_points = vec![Point::new(0.0, 0.0)];

        let geo_points = euclidean_to_geo(origin, &euclidean_points);

        assert_eq!(geo_points.len(), 1);
        assert!((geo_points[0].x() - origin.x()).abs() < 1e-10);
        assert!((geo_points[0].y() - origin.y()).abs() < 1e-10);
    }

    #[test]
    fn test_cardinal_directions() {
        let origin = Point::new(145.0, -37.0);
        let euclidean_points = vec![
            Point::new(1000.0, 0.0),  // East
            Point::new(0.0, 1000.0),  // North
            Point::new(-1000.0, 0.0), // West
            Point::new(0.0, -1000.0), // South
        ];

        let geo_points = euclidean_to_geo(origin, &euclidean_points);
        assert_eq!(geo_points.len(), 4);

        // East point should have greater longitude
        assert!(geo_points[0].x() > origin.x());
        // North point should have greater latitude
        assert!(geo_points[1].y() > origin.y());
        // West point should have lesser longitude
        assert!(geo_points[2].x() < origin.x());
        // South point should have lesser latitude
        assert!(geo_points[3].y() < origin.y());
    }
}
