#[derive(Debug, Default, Clone)]
pub struct SegmentSummary {
    pub distance_m: f64,
    pub elevation: Option<ElevationSummary>,
}

#[derive(Debug, Default, Clone)]
pub struct ElevationSummary {
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}
