use howitt::services::num::Round2;

#[derive(Round2, Debug, PartialEq)]
pub struct SegmentSummary {
    pub distance_m: f64,
    pub elevation: Option<ElevationSummary>,
}

#[derive(Round2, Debug, PartialEq)]
pub struct ElevationSummary {
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}

#[test]
fn test_round2() {
    let input = SegmentSummary {
        distance_m: 110.234,
        elevation: Some(ElevationSummary {
            elevation_ascent_m: 200.1,
            elevation_descent_m: 500.377,
        }),
    };

    let expected = SegmentSummary {
        distance_m: 110.23,
        elevation: Some(ElevationSummary {
            elevation_ascent_m: 200.1,
            elevation_descent_m: 500.38,
        }),
    };

    assert_eq!(input.round2(), expected)
}
