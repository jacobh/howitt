// src/lib/howitt/src/models/osm_feature.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ride::RideId, route::RouteId, Model, ModelName, ModelUuid};

pub type OsmFeatureId = ModelUuid<{ ModelName::OsmFeature }>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OsmFeature {
    pub id: OsmFeatureId,
    pub properties: HashMap<String, String>,
    pub geometry: geo::Geometry,
    pub created_at: DateTime<Utc>,
}

impl OsmFeature {
    pub fn id(&self) -> OsmFeatureId {
        self.id
    }
}

impl Model for OsmFeature {
    type Id = OsmFeatureId;
    type Filter = OsmFeatureFilter;

    fn id(&self) -> Self::Id {
        self.id()
    }
}

#[derive(Debug, Clone)]
pub enum OsmFeatureFilter {
    All,
    Id(OsmFeatureId),
    NearPoint {
        point: geo::Point,
        max_distance_meters: f64,
        limit: Option<usize>,
    },
    SimilarToGeometry {
        geometry: geo::Geometry,
        limit: Option<usize>,
        is_highway: bool,
    },
    IntersectsRide {
        ride_id: RideId,
    },
    IntersectsRoute {
        route_id: RouteId,
    },
}
