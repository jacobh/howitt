use itertools::{Itertools, Position};

use crate::models::{
    cuesheet::{Cue, CueStop, Cuesheet},
    point::{generate_point_deltas, ElevationPoint},
    point_of_interest::PointOfInterest,
};

use super::{
    nearby::{nearby_points_of_interest, NearbyPointOfInterest},
    summarize_segment::summarize_segment,
};

pub fn generate_cuesheet(route: &[ElevationPoint], pois: &[PointOfInterest]) -> Cuesheet {
    let nearby_pois = nearby_points_of_interest(route, pois, 500.0);

    let partitioned_points = route
        .iter()
        .with_position()
        .map(|(position, point)| match position {
            Position::First | Position::Middle => (point, false),
            Position::Last | Position::Only => (point, true),
        })
        .scan::<Vec<&ElevationPoint>, _, _>(vec![], |state, (point, is_last)| {
            state.push(point);

            let nearby_poi = nearby_pois
                .iter()
                .find(|nearby| nearby.closest_point.as_ref() == point);

            match nearby_poi {
                Some(nearby_poi) => {
                    let points = std::mem::take(state);
                    Some(Some((points, Some(nearby_poi.clone()))))
                }
                None => {
                    if is_last {
                        let points = std::mem::take(state);
                        Some(Some((points, None)))
                    } else {
                        Some(None)
                    }
                }
            }
        })
        .flatten();

    let summarized_partitioned_points = partitioned_points.map(|(points, poi)| {
        let summary = summarize_segment(&generate_point_deltas(&points));
        (points, poi, summary)
    });

    let cues = summarized_partitioned_points
        .scan::<Option<NearbyPointOfInterest<_>>, _, _>(None, |prev_poi, (_, poi, summary)| {
            let cue = Cue {
                origin: match prev_poi {
                    Some(poi) => CueStop::POI(poi.point_of_interest.clone().into_owned()),
                    None => CueStop::Start,
                },
                destination: match &poi {
                    Some(poi) => CueStop::POI(poi.point_of_interest.clone().into_owned()),
                    None => CueStop::End,
                },
                summary,
            };

            *prev_poi = poi;

            Some(cue)
        })
        .collect_vec();

    Cuesheet { cues }
}
