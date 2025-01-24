use crate::ext::iter::ScanAllExt;
use geo::{Bearing, Distance, Haversine};
use itertools::Itertools;

use super::{Point, WithDatetime, WithElevation};

pub trait Delta<P> {
    fn delta(value1: &P, value2: &P) -> Self;
}

pub trait AccumulatingDelta<P>: Sized {
    fn running_totals(values: &[P]) -> Vec<Self>;
}

// ----

#[derive(Debug)]
pub struct DistanceDelta(pub f64);
impl<P: Point> Delta<P> for DistanceDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        let distance = Haversine::distance(*value1.as_geo_point(), *value2.as_geo_point());
        DistanceDelta(distance)
    }
}

impl<P: Point> AccumulatingDelta<P> for DistanceDelta {
    fn running_totals(values: &[P]) -> Vec<Self> {
        std::iter::once(DistanceDelta(0.0))
            .chain(
                values
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| Self::delta(a, b)),
            )
            .scan_all(0.0, |acc, DistanceDelta(d)| {
                *acc += d;
                DistanceDelta(*acc)
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct BearingDelta(pub f64);
impl<P: Point> Delta<P> for BearingDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        let bearing = Haversine::bearing(*value1.as_geo_point(), *value2.as_geo_point());
        BearingDelta(bearing)
    }
}

#[derive(Debug)]
pub struct ElevationDelta(pub f64);
impl<P: WithElevation> Delta<P> for ElevationDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        // Simple elevation difference
        let delta = value2.elevation() - value1.elevation();
        ElevationDelta(delta)
    }
}

#[derive(Debug)]
pub struct ElevationGainDelta(pub f64);

impl<P: WithElevation> AccumulatingDelta<P> for ElevationGainDelta {
    fn running_totals(values: &[P]) -> Vec<Self> {
        std::iter::once(ElevationGainDelta(0.0))
            .chain(values.iter().tuple_windows().map(|(a, b)| {
                let delta = b.elevation() - a.elevation();
                ElevationGainDelta(if delta > 0.0 { delta } else { 0.0 })
            }))
            .scan_all(0.0, |acc, ElevationGainDelta(d)| {
                *acc += d;
                ElevationGainDelta(*acc)
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ElevationLossDelta(pub f64);

impl<P: WithElevation> AccumulatingDelta<P> for ElevationLossDelta {
    fn running_totals(values: &[P]) -> Vec<Self> {
        std::iter::once(ElevationLossDelta(0.0))
            .chain(values.iter().tuple_windows().map(|(a, b)| {
                let delta = b.elevation() - a.elevation();
                ElevationLossDelta(if delta < 0.0 { -delta } else { 0.0 })
            }))
            .scan_all(0.0, |acc, ElevationLossDelta(d)| {
                *acc += d;
                ElevationLossDelta(*acc)
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ElapsedDelta(pub chrono::Duration);
impl<P: WithDatetime> Delta<P> for ElapsedDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        // Time difference between points
        let delta = value2.datetime().signed_duration_since(*value1.datetime());
        ElapsedDelta(delta)
    }
}

impl<P: WithDatetime> AccumulatingDelta<P> for ElapsedDelta {
    fn running_totals(values: &[P]) -> Vec<Self> {
        std::iter::once(ElapsedDelta(chrono::Duration::zero()))
            .chain(
                values
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| Self::delta(a, b)),
            )
            .scan_all(chrono::Duration::zero(), |acc, ElapsedDelta(e)| {
                *acc = *acc + e;
                ElapsedDelta(*acc)
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct SpeedDelta(pub f64); // Speed in km/h

impl<P: Point + WithDatetime> Delta<P> for SpeedDelta {
    fn delta(value1: &P, value2: &P) -> Self {
        let distance = DistanceDelta::delta(value1, value2);
        let elapsed = ElapsedDelta::delta(value1, value2);

        // Convert meters to kilometers and seconds to hours
        let kilometers = distance.0 / 1000.0;
        let hours = elapsed.0.num_seconds() as f64 / 3600.0;

        // Avoid division by zero
        let speed = if hours > 0.0 { kilometers / hours } else { 0.0 };

        SpeedDelta(speed)
    }
}

// ----------

impl<P, T1> Delta<P> for (T1,)
where
    T1: Delta<P>,
{
    fn delta(value1: &P, value2: &P) -> Self {
        (T1::delta(value1, value2),)
    }
}

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

impl<P, T1, T2, T3, T4> Delta<P> for (T1, T2, T3, T4)
where
    T1: Delta<P>,
    T2: Delta<P>,
    T3: Delta<P>,
    T4: Delta<P>,
{
    fn delta(value1: &P, value2: &P) -> Self {
        (
            T1::delta(value1, value2),
            T2::delta(value1, value2),
            T3::delta(value1, value2),
            T4::delta(value1, value2),
        )
    }
}

impl<P, T1, T2, T3, T4, T5> Delta<P> for (T1, T2, T3, T4, T5)
where
    T1: Delta<P>,
    T2: Delta<P>,
    T3: Delta<P>,
    T4: Delta<P>,
    T5: Delta<P>,
{
    fn delta(value1: &P, value2: &P) -> Self {
        (
            T1::delta(value1, value2),
            T2::delta(value1, value2),
            T3::delta(value1, value2),
            T4::delta(value1, value2),
            T5::delta(value1, value2),
        )
    }
}

impl<P, T1> AccumulatingDelta<P> for (T1,)
where
    T1: AccumulatingDelta<P>,
{
    fn running_totals(values: &[P]) -> Vec<Self> {
        T1::running_totals(values)
            .into_iter()
            .map(|t1| (t1,))
            .collect()
    }
}

impl<P, T1, T2> AccumulatingDelta<P> for (T1, T2)
where
    T1: AccumulatingDelta<P>,
    T2: AccumulatingDelta<P>,
{
    fn running_totals(values: &[P]) -> Vec<Self> {
        let t1_deltas = T1::running_totals(values);
        let t2_deltas = T2::running_totals(values);
        t1_deltas.into_iter().zip(t2_deltas).collect()
    }
}

impl<P, T1, T2, T3> AccumulatingDelta<P> for (T1, T2, T3)
where
    T1: AccumulatingDelta<P>,
    T2: AccumulatingDelta<P>,
    T3: AccumulatingDelta<P>,
{
    fn running_totals(values: &[P]) -> Vec<Self> {
        let t1_deltas = T1::running_totals(values);
        let t2_deltas = T2::running_totals(values);
        let t3_deltas = T3::running_totals(values);
        t1_deltas
            .into_iter()
            .zip(t2_deltas)
            .zip(t3_deltas)
            .map(|((t1, t2), t3)| (t1, t2, t3))
            .collect()
    }
}

impl<P, T1, T2, T3, T4> AccumulatingDelta<P> for (T1, T2, T3, T4)
where
    T1: AccumulatingDelta<P>,
    T2: AccumulatingDelta<P>,
    T3: AccumulatingDelta<P>,
    T4: AccumulatingDelta<P>,
{
    fn running_totals(values: &[P]) -> Vec<Self> {
        let t1_deltas = T1::running_totals(values);
        let t2_deltas = T2::running_totals(values);
        let t3_deltas = T3::running_totals(values);
        let t4_deltas = T4::running_totals(values);
        t1_deltas
            .into_iter()
            .zip(t2_deltas)
            .zip(t3_deltas)
            .zip(t4_deltas)
            .map(|(((t1, t2), t3), t4)| (t1, t2, t3, t4))
            .collect()
    }
}

impl<P, T1, T2, T3, T4, T5> AccumulatingDelta<P> for (T1, T2, T3, T4, T5)
where
    T1: AccumulatingDelta<P>,
    T2: AccumulatingDelta<P>,
    T3: AccumulatingDelta<P>,
    T4: AccumulatingDelta<P>,
    T5: AccumulatingDelta<P>,
{
    fn running_totals(values: &[P]) -> Vec<Self> {
        let t1_deltas = T1::running_totals(values);
        let t2_deltas = T2::running_totals(values);
        let t3_deltas = T3::running_totals(values);
        let t4_deltas = T4::running_totals(values);
        let t5_deltas = T5::running_totals(values);
        t1_deltas
            .into_iter()
            .zip(t2_deltas)
            .zip(t3_deltas)
            .zip(t4_deltas)
            .zip(t5_deltas)
            .map(|((((t1, t2), t3), t4), t5)| (t1, t2, t3, t4, t5))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use geo::Point as GeoPoint;

    use super::*;
    use crate::models::point::{
        elevation_point::ElevationPoint, temporal_elevation_point::TemporalElevationPoint,
    };

    #[test]
    fn test_distance_delta() {
        let points = vec![
            ElevationPoint {
                point: GeoPoint::new(145.0, -37.0),
                elevation: 100.0,
            },
            ElevationPoint {
                point: GeoPoint::new(145.1, -37.1),
                elevation: 200.0,
            },
            ElevationPoint {
                point: GeoPoint::new(145.2, -37.2),
                elevation: 300.0,
            },
        ];

        let deltas = DistanceDelta::running_totals(&points);
        insta::assert_debug_snapshot!((points, deltas));
    }

    #[test]
    fn test_elevation_gain_loss_delta() {
        let points = vec![
            ElevationPoint {
                point: GeoPoint::new(145.0, -37.0),
                elevation: 100.0,
            },
            ElevationPoint {
                point: GeoPoint::new(145.1, -37.1),
                elevation: 200.0,
            },
            ElevationPoint {
                point: GeoPoint::new(145.2, -37.2),
                elevation: 150.0,
            },
            ElevationPoint {
                point: GeoPoint::new(145.3, -37.3),
                elevation: 300.0,
            },
        ];

        let gains = ElevationGainDelta::running_totals(&points);
        let losses = ElevationLossDelta::running_totals(&points);

        insta::assert_debug_snapshot!((points, gains, losses));
    }

    #[test]
    fn test_temporal_deltas() {
        let points = vec![
            TemporalElevationPoint {
                point: GeoPoint::new(145.0, -37.0),
                elevation: 100.0,
                datetime: Utc.timestamp_opt(1000, 0).unwrap(),
            },
            TemporalElevationPoint {
                point: GeoPoint::new(145.1, -37.1),
                elevation: 200.0,
                datetime: Utc.timestamp_opt(2000, 0).unwrap(),
            },
            TemporalElevationPoint {
                point: GeoPoint::new(145.2, -37.2),
                elevation: 150.0,
                datetime: Utc.timestamp_opt(3500, 0).unwrap(),
            },
        ];

        let deltas = <(ElapsedDelta, DistanceDelta, ElevationGainDelta)>::running_totals(&points);
        insta::assert_debug_snapshot!((points, deltas));
    }

    #[test]
    fn test_single_point_delta() {
        let p1 = ElevationPoint {
            point: GeoPoint::new(145.0, -37.0),
            elevation: 100.0,
        };
        let p2 = ElevationPoint {
            point: GeoPoint::new(145.1, -37.1),
            elevation: 200.0,
        };

        let distance = DistanceDelta::delta(&p1, &p2);
        let elevation = ElevationDelta::delta(&p1, &p2);
        let bearing = BearingDelta::delta(&p1, &p2);

        insta::assert_debug_snapshot!(((p1, p2), distance, elevation, bearing));
    }

    #[test]
    fn test_speed_delta() {
        let points = vec![
            TemporalElevationPoint {
                point: GeoPoint::new(145.0, -37.0),
                elevation: 100.0,
                datetime: Utc.timestamp_opt(0, 0).unwrap(),
            },
            // Point 1km away after 1 hour = 1 km/h
            TemporalElevationPoint {
                point: GeoPoint::new(145.009, -37.0), // Approximately 1km east
                elevation: 100.0,
                datetime: Utc.timestamp_opt(3600, 0).unwrap(),
            },
            // Point 2km away after 30 minutes = 4 km/h
            TemporalElevationPoint {
                point: GeoPoint::new(145.027, -37.0), // Approximately 3km east total
                elevation: 100.0,
                datetime: Utc.timestamp_opt(5400, 0).unwrap(),
            },
        ];

        let speed_deltas: Vec<_> = points
            .windows(2)
            .map(|window| SpeedDelta::delta(&window[0], &window[1]))
            .collect();

        insta::assert_debug_snapshot!(speed_deltas);
    }

    #[test]
    fn test_speed_delta_zero_time() {
        let p1 = TemporalElevationPoint {
            point: GeoPoint::new(145.0, -37.0),
            elevation: 100.0,
            datetime: Utc.timestamp_opt(0, 0).unwrap(),
        };
        let p2 = TemporalElevationPoint {
            point: GeoPoint::new(145.1, -37.1),
            elevation: 200.0,
            datetime: Utc.timestamp_opt(0, 0).unwrap(), // Same timestamp
        };

        let speed = SpeedDelta::delta(&p1, &p2);
        assert_eq!(speed.0, 0.0, "Speed should be 0 when time difference is 0");
    }

    #[test]
    fn test_speed_delta_with_distance_elapsed() {
        let points = vec![
            TemporalElevationPoint {
                point: GeoPoint::new(145.0, -37.0),
                elevation: 100.0,
                datetime: Utc.timestamp_opt(0, 0).unwrap(),
            },
            TemporalElevationPoint {
                point: GeoPoint::new(145.009, -37.0), // ~1km
                elevation: 100.0,
                datetime: Utc.timestamp_opt(900, 0).unwrap(), // 15 minutes = 4 km/h
            },
            TemporalElevationPoint {
                point: GeoPoint::new(145.018, -37.0), // ~2km
                elevation: 100.0,
                datetime: Utc.timestamp_opt(1500, 0).unwrap(), // 10 minutes = 12 km/h
            },
        ];

        // Test combined distance, time, and speed calculations
        let deltas1 = <(DistanceDelta, ElapsedDelta, SpeedDelta)>::delta(&points[0], &points[1]);
        let deltas2 = <(DistanceDelta, ElapsedDelta, SpeedDelta)>::delta(&points[1], &points[2]);
        insta::assert_debug_snapshot!((deltas1, deltas2));
    }
}
