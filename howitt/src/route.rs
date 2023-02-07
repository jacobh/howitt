use serde::{Deserialize, Serialize};

use crate::{external_ref::ExternalRef, point::ElevationPoint};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: ulid::Ulid,
    pub name: String,
    pub distance: f64,
    pub points: Vec<ElevationPoint>,
    pub external_ref: Option<ExternalRef>,
}

impl Route {
    pub fn iter_geo_points(&self) -> impl Iterator<Item = geo::Point> + '_ {
        self.points.iter().map(|point| point.point.clone())
    }
}
