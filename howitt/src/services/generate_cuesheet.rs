use itertools::{Itertools, Position};

use crate::models::{
    cuesheet::Cuesheet, point::ElevationPoint, point_of_interest::PointOfInterest,
};

use super::nearby::nearby_points_of_interest;

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
                    Some((points, Some(nearby_poi)))
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

    Cuesheet { cues: vec![] }
}
