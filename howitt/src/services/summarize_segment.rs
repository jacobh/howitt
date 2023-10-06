use crate::models::{point::PointDelta, segment_summary::SegmentSummary};

pub fn summarize_segment(point_deltas: &[PointDelta]) -> SegmentSummary {
    let summary = SegmentSummary {
        distance_m: 0.0,
        elevation: None,
    };

    let summary = point_deltas.iter().fold::<SegmentSummary, _>(
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

                if *elevation_gain > 0.0 {
                    elevation_summary.elevation_ascent_m += elevation_gain;
                } else {
                    elevation_summary.elevation_descent_m += elevation_gain.abs();
                }

                summary.elevation = Some(elevation_summary);
            }

            summary
        },
    );

    SegmentSummary {
        distance_m: f64::round(summary.distance_m * 100.0) / 100.0,
        ..summary
    }
}

#[cfg(test)]
mod tests {
    use crate::models::point::{generate_point_deltas, ElevationPoint};

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

        insta::assert_debug_snapshot!(result);
    }
}
