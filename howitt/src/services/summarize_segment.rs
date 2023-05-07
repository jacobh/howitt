use geo::GeodesicDistance;
use itertools::Itertools;

use crate::models::{point::Point, segment_summary::SegmentSummary};

pub fn summarize_segment<P: Point>(points: &[P]) -> SegmentSummary {
    points
        .iter()
        .tuple_windows()
        .map(|(p1, p2)| {
            (
                p1.as_geo_point().geodesic_distance(p2.as_geo_point()),
                match (p1.elevation_meters(), p2.elevation_meters()) {
                    (Some(e1), Some(e2)) => Some(e2 - e1),
                    _ => None,
                },
            )
        })
        .fold(
            SegmentSummary::default(),
            |mut summary, (distance, elevation)| {
                summary.distance_m += distance;

                if let Some(elevation) = elevation {
                    let mut elevation_summary = summary.elevation.unwrap_or_default();

                    if elevation > 0.0 {
                        elevation_summary.elevation_ascent_m += elevation;
                    } else {
                        elevation_summary.elevation_descent_m += elevation.abs();
                    }

                    summary.elevation = Some(elevation_summary);
                }

                summary
            },
        )
}
