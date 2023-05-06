use crate::models::{point::{ElevationPoint, Point}, point_of_interest::PointOfInterest, cuesheet::Cuesheet};

use super::nearby::nearby_points_of_interest;

pub fn generate_cuesheet(route: &[Point], pois: &[PointOfInterest]) -> Cuesheet {
    let nearby_pois = nearby_points_of_interest(route, pois);

    Cuesheet { cues: vec![] }
}