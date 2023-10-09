use crate::models::{
    point::{DeltaData, PointDelta},
    segment_summary::SegmentSummary,
};

pub fn summarize_segment<T: DeltaData>(point_deltas: &[PointDelta<T>]) -> SegmentSummary {
    let summary = SegmentSummary {
        distance_m: 0.0,
        elevation: None,
    };

    point_deltas.iter().fold::<SegmentSummary, _>(
        summary,
        |mut summary, PointDelta { distance, data, .. }| {
            summary.distance_m += distance;

            if let Some(elevation_gain) = data.elevation_gain() {
                let mut elevation_summary = summary.elevation.unwrap_or_default();

                if *elevation_gain > 0.0 {
                    elevation_summary.elevation_ascent_m += elevation_gain;
                } else {
                    elevation_summary.elevation_descent_m += elevation_gain.abs();
                }

                summary.elevation = Some(elevation_summary);
            }

            summary
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::models::point::{generate_point_deltas, ElevationPoint};
    use crate::services::num::Round2;

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

        let result = summarize_segment(&generate_point_deltas(&[point1.clone(), point2.clone()]));

        insta::assert_debug_snapshot!(result.round2());
    }
}
