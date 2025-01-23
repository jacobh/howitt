use serde::{Deserialize, Serialize};

use super::{delta2::*, Point, WithDatetime, WithElevation};

pub trait Progress: Sized {
    type Point: Point;

    fn from_points(points: Vec<Self::Point>) -> Vec<Self>;
    fn last_from_points(points: Vec<Self::Point>) -> Option<Self> {
        Self::from_points(points).into_iter().last()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistanceProgress<P: Point> {
    pub distance_m: f64,
    pub point: P,
}

impl<P: Point> Progress for DistanceProgress<P> {
    type Point = P;

    fn from_points(points: Vec<Self::Point>) -> Vec<Self> {
        let distance_deltas = DistanceDelta::running_totals(&points);

        points
            .into_iter()
            .zip(distance_deltas)
            .map(|(point, DistanceDelta(distance_m))| DistanceProgress { distance_m, point })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistanceElevationProgress<P: Point + WithElevation> {
    pub distance_m: f64,
    pub elevation_gain_m: f64,
    pub elevation_loss_m: f64,
    pub point: P,
}

impl<P: Point + WithElevation> Progress for DistanceElevationProgress<P> {
    type Point = P;

    fn from_points(points: Vec<Self::Point>) -> Vec<Self> {
        let deltas =
            <(DistanceDelta, ElevationGainDelta, ElevationLossDelta)>::running_totals(&points);

        points
            .into_iter()
            .zip(deltas)
            .map(
                |(
                    point,
                    (
                        DistanceDelta(distance_m),
                        ElevationGainDelta(elevation_gain_m),
                        ElevationLossDelta(elevation_loss_m),
                    ),
                )| {
                    DistanceElevationProgress {
                        distance_m,
                        elevation_gain_m,
                        elevation_loss_m,
                        point,
                    }
                },
            )
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TemporalDistanceProgress<P: Point + WithDatetime> {
    pub elapsed: chrono::Duration,
    pub distance_m: f64,
    pub point: P,
}

impl<P: Point + WithDatetime> Progress for TemporalDistanceProgress<P> {
    type Point = P;

    fn from_points(points: Vec<Self::Point>) -> Vec<Self> {
        let deltas = <(ElapsedDelta, DistanceDelta)>::running_totals(&points);

        points
            .into_iter()
            .zip(deltas)
            .map(
                |(point, (ElapsedDelta(elapsed), DistanceDelta(distance_m)))| {
                    TemporalDistanceProgress {
                        elapsed,
                        distance_m,
                        point,
                    }
                },
            )
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TemporalDistanceElevationProgress<P: Point + WithElevation + WithDatetime> {
    pub elapsed: chrono::Duration,
    pub distance_m: f64,
    pub elevation_gain_m: f64,
    pub elevation_loss_m: f64,
    pub point: P,
}

impl<P: Point + WithElevation + WithDatetime> Progress for TemporalDistanceElevationProgress<P> {
    type Point = P;

    fn from_points(points: Vec<Self::Point>) -> Vec<Self> {
        let deltas = <(
            ElapsedDelta,
            DistanceDelta,
            ElevationGainDelta,
            ElevationLossDelta,
        )>::running_totals(&points);

        points
            .into_iter()
            .zip(deltas)
            .map(
                |(
                    point,
                    (
                        ElapsedDelta(elapsed),
                        DistanceDelta(distance_m),
                        ElevationGainDelta(elevation_gain_m),
                        ElevationLossDelta(elevation_loss_m),
                    ),
                )| {
                    TemporalDistanceElevationProgress {
                        elapsed,
                        distance_m,
                        elevation_gain_m,
                        elevation_loss_m,
                        point,
                    }
                },
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use geo::Point as GeoPoint;
    use serde_json::json;

    use super::*;
    use crate::models::point::{
        elevation_point::ElevationPoint, temporal_elevation_point::TemporalElevationPoint,
    };

    #[test]
    fn test_distance_elevation_progress() {
        // Warby - Horsfall - McKinty - Jamieson - Bluff - Zeka - Bright
        // [lng, lat, elevation_m]
        let points = [
            [145.691096, -37.753237, 165.2988459925286],
            [145.814544, -37.767714, 612.9170135801276],
            [145.92525, -37.827911, 894.7186978043433],
            [145.915082, -37.773948, 762.6816635972889],
            [145.943995, -37.759184, 778.0513103936215],
            [146.021946, -37.775163, 917.9315929792149],
            [146.089115, -37.753413, 991.6369478982523],
            [146.135018, -37.788326, 1078.578038109006],
            [146.163214, -37.747364, 1099.3896065047804],
            [146.130649, -37.643917, 1179.3950521439506],
            [146.187036, -37.58, 1221.8075731293704],
            [146.250525, -37.610493, 1080.4347015026633],
            [146.301191, -37.59269, 1029.155117952875],
            [146.3677, -37.629382, 1072.6799660169956],
            [146.39131, -37.601278, 1331.9095925709023],
            [146.464612, -37.621102, 1130.1639908266504],
            [146.500166, -37.560111, 1236.8786280854638],
            [146.397056, -37.460475, 1141.9754003216833],
            [146.430459, -37.411493, 1225.3362181367347],
            [146.429612, -37.34351, 1375.8698176844523],
            [146.411502, -37.373503, 996.5273756239848],
            [146.332764, -37.309352, 489.753208094065],
            [146.259814, -37.317304, 406.307458066757],
            [146.217512, -37.288147, 354.1484139591959],
            [146.139432, -37.303228, 302.9304389010903],
            [146.146744, -37.266132, 305.44830685091694],
            [146.11898, -37.233163, 323.5966869453891],
            [146.242319, -37.23992, 360.47348053251284],
            [146.356789, -37.187204, 468.6202515078985],
            [146.425025, -37.197405, 593.2968446677794],
            [146.440833, -37.239275, 1061.1130846910244],
            [146.591895, -37.195311, 1616.262469875137],
            [146.605054, -37.215589, 1592.7607402193992],
            [146.69375, -37.212396, 1590.6501254481836],
            [146.707242, -37.178017, 1371.1008257392396],
            [146.73849, -37.185055, 1100.712318007096],
            [146.76097, -37.15443, 705.6623236980092],
            [146.785282, -37.167522, 594.064016065659],
            [146.799424, -37.113838, 976.1017221064632],
            [146.841945, -37.101877, 955.9089757934864],
            [146.832009, -37.070078, 1128.3327670216117],
            [146.889833, -37.054334, 1254.2326424244602],
            [146.878378, -37.024011, 1172.8360930969532],
            [146.916532, -37.011648, 685.5673069216674],
            [146.934499, -36.978547, 558.076184371075],
            [146.925271, -36.9308, 479.71879578343504],
            [146.851485, -36.846183, 405.39634720656016],
            [146.852624, -36.779067, 350.0891501048425],
            [146.907633, -36.700651, 276.40856773664586],
            [146.956456, -36.727756, 305.0141445201914],
        ];

        let points: Vec<ElevationPoint> = points
            .iter()
            .map(|[lng, lat, elevation]| ElevationPoint {
                point: GeoPoint::new(*lng, *lat),
                elevation: *elevation,
            })
            .collect();

        let progress = DistanceElevationProgress::from_points(points.clone());

        assert_eq!(points.len(), progress.len());

        insta::assert_debug_snapshot!(progress)
    }

    #[test]
    fn test_temporal_distance_elevation_progress() {
        // Warby - Horsfall - McKinty - Jamieson - Bluff - Zeka - Bright
        // [lng, lat, elevation_m, timestamp]
        let points = json!([
            [146.577011, -37.207081, 1448.0, "2024-12-23T11:56:36+11:00"],
            [146.5802, -37.197559, 1537.2, "2024-12-23T12:15:55+11:00"],
            [146.591812, -37.195274, 1591.4, "2024-12-23T12:35:31+11:00"],
            [146.604782, -37.215446, 1558.1, "2024-12-23T13:05:32+11:00"],
            [146.613174, -37.206764, 1453.0, "2024-12-23T13:24:41+11:00"],
            [146.615189, -37.211834, 1414.2, "2024-12-23T13:26:47+11:00"],
            [146.625626, -37.21172, 1229.0, "2024-12-23T13:34:06+11:00"],
            [146.621872, -37.215477, 1157.4, "2024-12-23T13:36:59+11:00"],
            [146.632965, -37.211624, 1048.2, "2024-12-23T14:06:59+11:00"],
            [146.644836, -37.216881, 996.4, "2024-12-23T14:22:38+11:00"],
            [146.662094, -37.201038, 1304.8, "2024-12-23T15:34:16+11:00"],
            [146.66188, -37.211628, 1482.6, "2024-12-23T16:04:13+11:00"],
            [146.681244, -37.204636, 1580.0, "2024-12-23T16:32:38+11:00"],
            [146.69574, -37.213554, 1563.4, "2024-12-23T16:51:19+11:00"],
            [146.691177, -37.201672, 1418.4, "2024-12-23T17:05:28+11:00"],
            [146.695526, -37.204445, 1356.2, "2024-12-23T17:09:16+11:00"],
            [146.699158, -37.199127, 1286.0, "2024-12-23T17:12:33+11:00"],
            [146.6996, -37.204128, 1155.8, "2024-12-23T17:45:48+11:00"],
            [146.703659, -37.197807, 1113.4, "2024-12-23T17:49:39+11:00"],
            [146.693695, -37.192905, 1033.8, "2024-12-23T18:02:36+11:00"],
            [146.707153, -37.177982, 1323.8, "2024-12-23T19:12:55+11:00"],
            [146.738373, -37.185024, 1052.8, "2024-12-23T19:44:45+11:00"],
            [146.760941, -37.154312, 642.2, "2024-12-23T20:13:37+11:00"],
            [146.775742, -37.16959, 504.4, "2024-12-23T20:30:56+11:00"],
            [146.782104, -37.168179, 490.0, "2024-12-23T20:37:55+11:00"],
        ]);

        let points: Vec<TemporalElevationPoint> = points
            .as_array()
            .unwrap()
            .iter()
            .map(|point| {
                let coords = point.as_array().unwrap();
                let lng = coords[0].as_f64().unwrap();
                let lat = coords[1].as_f64().unwrap();
                let elevation = coords[2].as_f64().unwrap();
                let timestamp = coords[3].as_str().unwrap();

                TemporalElevationPoint {
                    point: GeoPoint::new(lng, lat),
                    elevation,
                    datetime: DateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%S%z")
                        .unwrap()
                        .with_timezone(&Utc),
                }
            })
            .collect();

        let progress = TemporalDistanceElevationProgress::from_points(points.clone());

        assert_eq!(points.len(), progress.len());

        insta::assert_debug_snapshot!(progress)
    }
}
