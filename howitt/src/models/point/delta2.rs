use geo::{Bearing, Distance, Haversine};

use super::{Point, WithDatetime, WithElevation};

pub trait Delta<P>: Sized {
    fn delta(value1: &P, value2: &P) -> Self;
}

pub trait CumulativeDelta<P>: Delta<P> {
    fn cumulative_deltas(values: &[P]) -> Vec<Self>;
}

pub struct DistanceDelta(pub f64);
impl<P: Point> Delta<P> for DistanceDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        let distance = Haversine::distance(*value1.as_geo_point(), *value2.as_geo_point());
        DistanceDelta(distance)
    }
}

impl<P: Point> CumulativeDelta<P> for DistanceDelta {
    fn cumulative_deltas(values: &[P]) -> Vec<Self> {
        std::iter::once(DistanceDelta(0.0))
            .chain(values.windows(2).map(|w| Self::delta(&w[0], &w[1])))
            .scan(0.0, |acc, DistanceDelta(d)| {
                *acc += d;
                Some(DistanceDelta(*acc))
            })
            .collect()
    }
}

pub struct BearingDelta(pub f64);
impl<P: Point> Delta<P> for BearingDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        let bearing = Haversine::bearing(*value1.as_geo_point(), *value2.as_geo_point());
        BearingDelta(bearing)
    }
}

pub struct ElevationDelta(pub f64);
impl<P: WithElevation> Delta<P> for ElevationDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        // Simple elevation difference
        let delta = value2.elevation() - value1.elevation();
        ElevationDelta(delta)
    }
}

impl<P: WithElevation> CumulativeDelta<P> for ElevationDelta {
    fn cumulative_deltas(values: &[P]) -> Vec<Self> {
        std::iter::once(ElevationDelta(0.0))
            .chain(values.windows(2).map(|w| Self::delta(&w[0], &w[1])))
            .scan(0.0, |acc, ElevationDelta(e)| {
                *acc += e;
                Some(ElevationDelta(*acc))
            })
            .collect()
    }
}

pub struct ElapsedDelta(pub chrono::Duration);
impl<P: WithDatetime> Delta<P> for ElapsedDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        // Time difference between points
        let delta = value2.datetime().signed_duration_since(*value1.datetime());
        ElapsedDelta(delta)
    }
}

impl<P: WithDatetime> CumulativeDelta<P> for ElapsedDelta {
    fn cumulative_deltas(values: &[P]) -> Vec<Self> {
        std::iter::once(ElapsedDelta(chrono::Duration::zero()))
            .chain(values.windows(2).map(|w| Self::delta(&w[0], &w[1])))
            .scan(chrono::Duration::zero(), |acc, ElapsedDelta(e)| {
                *acc = *acc + e;
                Some(ElapsedDelta(*acc))
            })
            .collect()
    }
}

// Composite Deltas

pub struct DistanceElevationDelta {
    pub distance: DistanceDelta,
    pub elevation: ElevationDelta,
}

impl<P: Point + WithElevation> Delta<P> for DistanceElevationDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        DistanceElevationDelta {
            distance: DistanceDelta::delta(value1, value2),
            elevation: ElevationDelta::delta(value1, value2),
        }
    }
}

impl<P: Point + WithElevation> CumulativeDelta<P> for DistanceElevationDelta {
    fn cumulative_deltas(values: &[P]) -> Vec<Self> {
        let distances = DistanceDelta::cumulative_deltas(values);
        let elevations = ElevationDelta::cumulative_deltas(values);
        distances
            .into_iter()
            .zip(elevations)
            .map(|(distance, elevation)| DistanceElevationDelta {
                distance,
                elevation,
            })
            .collect()
    }
}

pub struct DistanceElapsedDelta {
    pub distance: DistanceDelta,
    pub elapsed: ElapsedDelta,
}

impl<P: Point + WithDatetime> Delta<P> for DistanceElapsedDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        DistanceElapsedDelta {
            distance: DistanceDelta::delta(value1, value2),
            elapsed: ElapsedDelta::delta(value1, value2),
        }
    }
}

impl<P: Point + WithDatetime> CumulativeDelta<P> for DistanceElapsedDelta {
    fn cumulative_deltas(values: &[P]) -> Vec<Self> {
        let distances = DistanceDelta::cumulative_deltas(values);
        let elapsed = ElapsedDelta::cumulative_deltas(values);
        distances
            .into_iter()
            .zip(elapsed)
            .map(|(distance, elapsed)| DistanceElapsedDelta { distance, elapsed })
            .collect()
    }
}

pub struct DistanceElevationElapsedDelta {
    pub distance: DistanceDelta,
    pub elevation: ElevationDelta,
    pub elapsed: ElapsedDelta,
}

impl<P: Point + WithElevation + WithDatetime> Delta<P> for DistanceElevationElapsedDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        DistanceElevationElapsedDelta {
            distance: DistanceDelta::delta(value1, value2),
            elevation: ElevationDelta::delta(value1, value2),
            elapsed: ElapsedDelta::delta(value1, value2),
        }
    }
}

impl<P: Point + WithElevation + WithDatetime> CumulativeDelta<P> for DistanceElevationElapsedDelta {
    fn cumulative_deltas(values: &[P]) -> Vec<Self> {
        let distances = DistanceDelta::cumulative_deltas(values);
        let elevations = ElevationDelta::cumulative_deltas(values);
        let elapsed = ElapsedDelta::cumulative_deltas(values);
        distances
            .into_iter()
            .zip(elevations)
            .zip(elapsed)
            .map(
                |((distance, elevation), elapsed)| DistanceElevationElapsedDelta {
                    distance,
                    elevation,
                    elapsed,
                },
            )
            .collect()
    }
}
