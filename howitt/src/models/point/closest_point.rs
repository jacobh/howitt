use super::{point::Point, point_delta::PointDelta};

pub fn closest_point<'a, P: Point>(
    point: &P,
    points: impl Iterator<Item = &'a P>,
) -> Option<(&'a P, PointDelta<P::DeltaData>)> {
    points
        .map(|p| {
            let delta = PointDelta::from_points(point, p);
            (p, delta)
        })
        .min_by_key(|(_, delta)| delta.clone())
}
