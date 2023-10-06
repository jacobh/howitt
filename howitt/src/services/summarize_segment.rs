use itertools::Itertools;

use crate::models::{
    point::{Point, PointDelta},
    segment_summary::SegmentSummary,
};

use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("Failed to summarize segment")]
pub enum SummarizeError {
    NotEnoughPoints,
}

pub fn summarize_segment<P: Point>(points: &[P]) -> Result<SegmentSummary, SummarizeError> {
    if points.len() < 2 {
        return Err(SummarizeError::NotEnoughPoints);
    }

    let summary = SegmentSummary {
        distance_m: 0.0,
        elevation: None,
    };

    let summary = points
        .iter()
        .tuple_windows()
        .map(PointDelta::from_points_tuple)
        // .flatten()
        .fold::<SegmentSummary, _>(
            summary,
            |mut summary,
             PointDelta {
                 distance,
                 elevation_gain,
                 ..
             }| {
                summary.distance_m += distance;

                if let Some(elevation_gain) = elevation_gain {
                    let mut elevation_summary = summary.elevation.unwrap_or_default();

                    if elevation_gain > 0.0 {
                        elevation_summary.elevation_ascent_m += elevation_gain;
                    } else {
                        elevation_summary.elevation_descent_m += elevation_gain.abs();
                    }

                    summary.elevation = Some(elevation_summary);
                }

                summary
            },
        );

    Ok(SegmentSummary {
        distance_m: f64::round(summary.distance_m * 100.0) / 100.0,
        ..summary
    })
}

#[cfg(test)]
mod tests {
    use crate::models::point::ElevationPoint;

    use super::*;

    #[test]
    fn it_succeeds_for_two_points() {
        let point1 = ElevationPoint {
            point: geo::Point::new(146.60587, -37.2154),
            elevation: 1100.0,
        };
        let point2 = ElevationPoint {
            point: geo::Point::new(146.68021, -37.20515),
            elevation: 1400.0,
        };

        let result =
            summarize_segment(&[point1.clone(), point2.clone()]).expect("summarize worked");

        insta::assert_debug_snapshot!(result);
    }
}
