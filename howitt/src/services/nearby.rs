use geo::algorithm::haversine_distance::HaversineDistance;

use crate::models::checkpoint::Checkpoint;
#[derive(Debug, Clone)]
pub struct NearbyCheckpoint<'checkpoint> {
    pub point_idx: usize,
    pub closest_point: geo::Point<f64>,
    pub distance: f64,
    pub checkpoint: &'checkpoint Checkpoint,
}

pub fn nearby_checkpoints<'c>(
    route: &gpx::Route,
    checkpoints: &'c [Checkpoint],
) -> Vec<NearbyCheckpoint<'c>> {
    checkpoints
        .iter()
        .filter_map(|checkpoint| {
            let closest_point = route
                .linestring()
                .into_iter()
                .map(geo::Point::from)
                .enumerate()
                .map(|(i, point)| (i, point, point.haversine_distance(&checkpoint.point)))
                .min_by_key(|(_, _, distance)| (distance * 10000.0) as i64);

            closest_point.map(|(point_idx, closest_point, distance)| NearbyCheckpoint {
                point_idx,
                closest_point,
                distance,
                checkpoint,
            })
        })
        .collect()
}
