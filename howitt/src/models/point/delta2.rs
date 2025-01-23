use geo::{Bearing, Distance, Haversine};

use super::{Point, WithDatetime, WithElevation};

pub trait Delta<P>: Sized {
    fn delta(value1: &P, value2: &P) -> Self;
}

pub trait CumulativeDelta<P>: Delta<P> {
    fn cumulative_deltas(values: &[P]) -> Vec<Self>;
}

// ----------

impl<P, T1, T2> Delta<P> for (T1, T2)
where
    T1: Delta<P>,
    T2: Delta<P>,
{
    fn delta(value1: &P, value2: &P) -> Self {
        (T1::delta(value1, value2), T2::delta(value1, value2))
    }
}

impl<P, T1, T2, T3> Delta<P> for (T1, T2, T3)
where
    T1: Delta<P>,
    T2: Delta<P>,
    T3: Delta<P>,
{
    fn delta(value1: &P, value2: &P) -> Self {
        (
            T1::delta(value1, value2),
            T2::delta(value1, value2),
            T3::delta(value1, value2),
        )
    }
}

impl<P, T1, T2> CumulativeDelta<P> for (T1, T2)
where
    T1: CumulativeDelta<P>,
    T2: CumulativeDelta<P>,
{
    fn cumulative_deltas(values: &[P]) -> Vec<Self> {
        let t1_deltas = T1::cumulative_deltas(values);
        let t2_deltas = T2::cumulative_deltas(values);
        t1_deltas.into_iter().zip(t2_deltas).collect()
    }
}

impl<P, T1, T2, T3> CumulativeDelta<P> for (T1, T2, T3)
where
    T1: CumulativeDelta<P>,
    T2: CumulativeDelta<P>,
    T3: CumulativeDelta<P>,
{
    fn cumulative_deltas(values: &[P]) -> Vec<Self> {
        let t1_deltas = T1::cumulative_deltas(values);
        let t2_deltas = T2::cumulative_deltas(values);
        let t3_deltas = T3::cumulative_deltas(values);
        t1_deltas
            .into_iter()
            .zip(t2_deltas)
            .zip(t3_deltas)
            .map(|((t1, t2), t3)| (t1, t2, t3))
            .collect()
    }
}

// ----------

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
