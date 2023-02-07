use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoutePoint {
    pub point: geo::Point<f64>,
    pub elevation: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub name: String,
    pub points: Vec<RoutePoint>
}
