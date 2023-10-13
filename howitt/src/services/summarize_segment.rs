use crate::models::{
    point::{DeltaData, PointDelta},
    segment_summary::{SegmentSummary, SummaryData},
};

pub fn summarize_segment<T: DeltaData>(
    point_deltas: &[PointDelta<T>],
) -> SegmentSummary<T::SummaryData> {
    let summary = SegmentSummary {
        distance_m: 0.0,
        data: T::SummaryData::default(),
    };

    point_deltas.iter().fold::<SegmentSummary<_>, _>(
        summary,
        |summary, PointDelta { distance, data, .. }| SegmentSummary {
            distance_m: summary.distance_m + distance,
            data: summary.data.fold(data.to_summary()),
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

        let result = summarize_segment(&generate_point_deltas(&[
            point1.clone(),
            point2.clone(),
            point1.clone(),
        ]));

        insta::assert_debug_snapshot!(result.round2());
    }
}
