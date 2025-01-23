use super::{
    delta::{Delta, DistanceDelta},
    point::Point,
};
use ordered_float::OrderedFloat;

pub fn closest_point<'a, P: Point>(
    point: &P,
    points: impl Iterator<Item = &'a P>,
) -> Option<(&'a P, DistanceDelta)> {
    points
        .map(|p| {
            let distance = DistanceDelta::delta(point, p);
            (p, distance)
        })
        .min_by_key(|(_, DistanceDelta(dist))| OrderedFloat(*dist))
}
