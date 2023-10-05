use geo::GeodesicDistance;

use crate::models::{point::Point, segment_summary::SegmentSummary};

pub fn summarize_segment<P: Point>(points: impl Iterator<Item = P>) -> SegmentSummary {
    points
        .scan::<Option<&P>, _, _>(None, |prev_point, point| match prev_point {
            Some(prev_point) => {
                let distance = prev_point
                    .as_geo_point()
                    .geodesic_distance(point.as_geo_point());

                let elevation = match (prev_point.elevation_meters(), point.elevation_meters()) {
                    (Some(e1), Some(e2)) => Some(e2 - e1),
                    _ => None,
                };

                *prev_point = &point;

                Some(Some((distance, elevation)))
            }
            None => {
                *prev_point = Some(&point);
                Some(None)
            }
        })
        .flatten()
        .fold::<SegmentSummary, _>(
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
