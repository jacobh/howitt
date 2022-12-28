use geo::Closest;
use geo::prelude::ClosestPoint;
use geo::algorithm::geodesic_distance::GeodesicDistance;

use crate::checkpoint::Checkpoint;
pub struct NearbyCheckpoint<'checkpoint> {
    pub closest_point: geo::Point<f64>,
    pub distance: f64,
    pub checkpoint: &'checkpoint Checkpoint
}


pub fn nearby_checkpoints<'r, 'c>(route: &'r gpx::Route, checkpoints: &'c [Checkpoint]) -> Vec<NearbyCheckpoint<'c>> {
    checkpoints.into_iter().filter_map(|checkpoint| {
        let closest_point = match route.linestring().closest_point(&checkpoint.point) {
            Closest::SinglePoint(point) | Closest::Intersection(point) => Some(point),
            Closest::Indeterminate => None
        };

        closest_point.map(|closest_point| {
            let distance = closest_point.geodesic_distance(&checkpoint.point);
    
            NearbyCheckpoint {
                closest_point, distance, checkpoint
            }
        })
    }).collect()
}