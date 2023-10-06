use either::Either;
use geo::GeodesicBearing;
use geo::GeodesicDistance;

use crate::models::{
    point::Point,
    segment_summary::{CardinalDirection, SegmentSummary, SlopeEnd, Termini, Terminus},
};

use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("Failed to summarize segment")]
pub enum SummarizeError {
    NotEnoughPoints,
}

pub fn summarize_segment<P: Point>(points: &[P]) -> Result<SegmentSummary<P>, SummarizeError> {
    if points.len() < 2 {
        return Err(SummarizeError::NotEnoughPoints);
    }

    let summary = SegmentSummary::<P> {
        distance_m: 0.0,
        elevation: None,
        termini: Termini::new(
            points.first().unwrap().clone(),
            points.last().unwrap().clone(),
        ),
    };

    Ok(points
        .iter()
        .scan::<Option<&P>, _, _>(None, |prev_point, point| match prev_point {
            Some(prev_point) => {
                let distance = prev_point
                    .as_geo_point()
                    .geodesic_distance(point.as_geo_point());

                let elevation = match (prev_point.elevation_meters(), point.elevation_meters()) {
                    (Some(e1), Some(e2)) => Some(e2 - e1),
                    _ => None,
                };

                *prev_point = point;

                Some(Some((distance, elevation)))
            }
            None => {
                *prev_point = Some(point);
                Some(None)
            }
        })
        .flatten()
        .fold::<SegmentSummary<P>, _>(summary, |mut summary, (distance, elevation)| {
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
        }))
}
