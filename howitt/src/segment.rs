use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{checkpoint::Checkpoint, nearby::nearby_checkpoints};

crate::model_id!(SegmentId, "SEGMENT");

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: SegmentId,
    pub start: Checkpoint,
    pub end: Checkpoint,
    pub route: gpx::Route,
}

pub fn detect_segments(route: &gpx::Route, checkpoints: &[Checkpoint]) -> Vec<Segment> {
    nearby_checkpoints(route, checkpoints)
        .into_iter()
        .sorted_by_key(|cp| cp.point_idx)
        .filter(|cp| cp.distance < 500.0)
        .combinations(2)
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .map(|(cp1, cp2)| Segment {
            id: SegmentId::new(),
            start: cp1.checkpoint.clone(),
            end: cp2.checkpoint.clone(),
            route: gpx::Route {
                points: route.points[cp1.point_idx..=cp2.point_idx].to_vec(),
                ..route.clone()
            },
        })
        .collect()
}
