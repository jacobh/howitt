use super::{delta2::*, Point, WithDatetime, WithElevation};

pub trait Progress: Sized {
    type Point: Point;

    fn from_points(points: Vec<Self::Point>) -> Vec<Self>;
}

pub struct DistanceProgress<P: Point> {
    pub distance_m: f64,
    pub point: P,
}

pub struct DistanceElevationProgress<P: Point + WithElevation> {
    pub distance_m: f64,
    pub elevation_gain_m: f64,
    pub elevation_loss_m: f64,
    pub point: P,
}

pub struct TemporalDistanceProgress<P: Point + WithDatetime> {
    pub elapsed: chrono::Duration,
    pub distance_m: f64,
    pub point: P,
}

pub struct TemporalDistanceElevationProgress<P: Point + WithElevation + WithDatetime> {
    pub elapsed: chrono::Duration,
    pub distance_m: f64,
    pub elevation_gain_m: f64,
    pub elevation_loss_m: f64,
    pub point: P,
}
