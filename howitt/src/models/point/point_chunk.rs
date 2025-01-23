use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::models::ModelId;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointChunk<ID, P> {
    pub model_id: ID,
    pub idx: usize,
    pub points: Vec<P>,
}
impl<ID, P> PointChunk<ID, P>
where
    P: 'static,
    ID: ModelId,
{
    pub fn new_chunks(model_id: ID, points: impl IntoIterator<Item = P>) -> Vec<PointChunk<ID, P>> {
        points
            .into_iter()
            .chunks(2500)
            .into_iter()
            .enumerate()
            .map(|(idx, points)| PointChunk {
                model_id,
                idx,
                points: points.collect(),
            })
            .collect()
    }
    pub fn iter_points(chunks: &[PointChunk<ID, P>]) -> impl Iterator<Item = &P> + '_ {
        chunks.iter().flat_map(|chunk| chunk.points.iter())
    }
    pub fn into_iter_points(chunks: Vec<PointChunk<ID, P>>) -> impl Iterator<Item = P> + 'static {
        chunks
            .into_iter()
            .flat_map(|chunk| chunk.points.into_iter())
    }
}
