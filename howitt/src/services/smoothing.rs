pub fn smooth_elevations(cum_distances: &[f64], elevations: &[f64]) -> Vec<f64> {
    let spline = csaps::CubicSmoothingSpline::new(&cum_distances, &elevations)
        .with_smooth(0.000002)
        .make()
        .unwrap();

    let smoothed_elevations = spline.evaluate(&cum_distances).unwrap();

    smoothed_elevations.to_vec()
}
