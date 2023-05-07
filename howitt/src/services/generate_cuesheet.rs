use itertools::{Itertools, Position};

use crate::models::{
    cuesheet::{Cue, CueStop, Cuesheet},
    point::ElevationPoint,
    point_of_interest::PointOfInterest,
    segment_summary::SegmentSummary,
};

use super::{
    nearby::{nearby_points_of_interest, NearbyPointOfInterest},
    summarize_segment::summarize_segment,
};

pub fn generate_cuesheet<P>(route: &[ElevationPoint], pois: &[PointOfInterest]) -> Cuesheet {
    let nearby_pois = nearby_points_of_interest(route, pois, 500.0);

    let partitioned_points = route
        .iter()
        .with_position()
        .map(|point| match point {
            Position::First(point) | Position::Middle(point) => (point, false),
            Position::Last(point) | Position::Only(point) => (point, true),
        })
        .scan::<Vec<&ElevationPoint>, _, _>(vec![], |state, (point, is_last)| {
            state.push(point);

            let nearby_poi = nearby_pois
                .iter()
                .find(|nearby| nearby.closest_point.as_ref() == point);

            match nearby_poi {
                Some(nearby_poi) => {
                    let points = std::mem::replace(state, vec![]);
                    Some((points, Some(nearby_poi.clone())))
                }
                None => {
                    if is_last {
                        let points = std::mem::replace(state, vec![]);
                        Some((points, None))
                    } else {
                        None
                    }
                }
            }
        });

    let summarized_partitioned_points = partitioned_points.map(|(points, poi)| {
        let summary = summarize_segment(&points);
        (points, poi, summary)
    });

    let cues = summarized_partitioned_points
        .scan::<Option<(_, Option<NearbyPointOfInterest<_>>, SegmentSummary)>, _, _>(
            None,
            |prev_segment, (points, poi, summary)| {
                let prev_poi = prev_segment.as_ref().and_then(|(_, poi, _)| poi.as_ref());

                let cue = Cue {
                    origin: match prev_poi {
                        Some(poi) => CueStop::POI(poi.point_of_interest.clone().into_owned()),
                        None => CueStop::Start,
                    },
                    destination: match &poi {
                        Some(poi) => CueStop::POI(poi.point_of_interest.clone().into_owned()),
                        None => CueStop::End,
                    },
                    distance_m: summary.distance_m,
                    vertical_ascent_m: summary
                        .elevation
                        .as_ref()
                        .map(|elev| elev.elevation_ascent_m),
                    vertical_descent_m: summary
                        .elevation
                        .as_ref()
                        .map(|elev| elev.elevation_descent_m),
                };

                *prev_segment = Some((points, poi, summary));

                Some(cue)
            },
        )
        .collect_vec();

    Cuesheet { cues }
}
