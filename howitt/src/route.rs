use serde::{Serialize, Deserialize};

use crate::point::ElevationPoint;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub name: String,
    pub points: Vec<ElevationPoint>
}
