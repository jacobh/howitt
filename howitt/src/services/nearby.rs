use crate::models::point_of_interest::PointOfInterest;
use geo::algorithm::haversine_distance::HaversineDistance;

#[derive(Debug, Clone)]
pub struct NearbyPointOfInterest<'poi> {
    pub point_idx: usize,
    pub closest_point: geo::Point<f64>,
    pub distance: f64,
    pub point_of_interest: &'poi PointOfInterest,
}

pub fn nearby_points_of_interest<'c>(
    route: &gpx::Route,
    pois: &'c [PointOfInterest],
) -> Vec<NearbyPointOfInterest<'c>> {
    pois
        .iter()
        .filter_map(|poi| {
            let closest_point = route
                .linestring()
                .into_iter()
                .map(geo::Point::from)
                .enumerate()
                .map(|(i, point)| (i, point, point.haversine_distance(&poi.point)))
                .min_by_key(|(_, _, distance)| (distance * 10000.0) as i64);

            closest_point.map(|(point_idx, closest_point, distance)| NearbyPointOfInterest {
                point_idx,
                closest_point,
                distance,
                point_of_interest: poi,
            })
        })
        .collect()
}
