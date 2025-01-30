use itertools::{Itertools, Position};

use crate::ext::iter::ScanAllExt;

use crate::models::{
    cuesheet::{Cue, CueStop, Cuesheet},
    point::{
        progress::{DistanceElevationProgress, Progress},
        ElevationPoint,
    },
    point_of_interest::PointOfInterest,
};

use super::nearby::{nearby_points_of_interest, NearbyPointOfInterest};

pub fn generate_cuesheet(route: &[ElevationPoint], pois: &[PointOfInterest]) -> Cuesheet {
    let nearby_pois = nearby_points_of_interest(route, pois, 500.0);

    let partitioned_points = route
        .iter()
        .with_position()
        .map(|(position, point)| match position {
            Position::First | Position::Middle => (point, false),
            Position::Last | Position::Only => (point, true),
        })
        .scan_all(vec![], |state, (point, is_last)| {
            state.push(point);

            let nearby_poi = nearby_pois
                .iter()
                .find(|nearby| nearby.closest_point.as_ref() == point);

            match nearby_poi {
                Some(nearby_poi) => {
                    let points = std::mem::take(state);
                    Some((points, Some(nearby_poi.clone())))
                }
                None => {
                    if is_last {
                        let points = std::mem::take(state);
                        Some((points, None))
                    } else {
                        None
                    }
                }
            }
        })
        .flatten();

    let summarized_partitioned_points =
        partitioned_points.map(|(points, poi): (Vec<&ElevationPoint>, _)| {
            // Convert points to progress and get the last one for the summary
            let progress = DistanceElevationProgress::last_from_points(
                points.clone().into_iter().cloned().collect(),
            )
            .expect("Should have at least one point");

            (points, poi, progress)
        });

    let cues = summarized_partitioned_points
        .scan_all(
            None,
            |prev_poi: &mut Option<NearbyPointOfInterest<_>>, (_, poi, summary)| {
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
                cue
            },
        )
        .collect_vec();

    Cuesheet { cues }
}
