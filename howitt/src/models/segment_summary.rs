use either::Either;
use serde::{Deserialize, Serialize};

use super::point::Point;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlopeEnd {
    Uphill,
    Downhill,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terminus<P: Point> {
    pub direction: CardinalDirection,
    pub slope_end: SlopeEnd,
    pub point: P,
}

pub type Termini<P> = Either<P, (Terminus<P>, Terminus<P>)>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentSummary<P: Point> {
    pub distance_m: f64,
    pub elevation: Option<ElevationSummary>,
    pub termini: Termini<P>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ElevationSummary {
    pub elevation_ascent_m: f64,
    pub elevation_descent_m: f64,
}
