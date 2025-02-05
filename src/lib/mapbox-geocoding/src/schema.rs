use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeatureType {
    Country,
    Region,
    Postcode,
    District,
    Place,
    Locality,
    Neighborhood,
    Street,
    Block,
    Address,
    SecondaryAddress,
}

// Query Parameters for Forward Geocoding
#[derive(Debug, Serialize)]
pub struct ForwardGeocodingParams {
    pub q: String,
    pub access_token: String,
    pub permanent: Option<bool>,
    pub autocomplete: Option<bool>,
    pub bbox: Option<[f64; 4]>, // [minLon,minLat,maxLon,maxLat]
    pub country: Option<String>,
    pub format: Option<String>, // "geojson" or "v5"
    pub language: Option<String>,
    pub limit: Option<u8>,
    pub proximity: Option<String>,
    pub types: Option<FeatureType>,
    pub worldview: Option<String>,
}

// Query Parameters for Structured Forward Geocoding
#[derive(Debug, Serialize)]
pub struct StructuredGeocodingParams {
    pub access_token: String,
    pub address_line1: Option<String>,
    pub address_number: Option<String>,
    pub street: Option<String>,
    pub block: Option<String>,
    pub place: Option<String>,
    pub region: Option<String>,
    pub postcode: Option<String>,
    pub locality: Option<String>,
    pub neighborhood: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReverseGeocodingParams {
    pub longitude: f64,
    pub latitude: f64,
    pub access_token: String,
    pub permanent: Option<bool>,
    pub country: Option<String>,
    pub language: Option<String>,
    pub limit: Option<u8>,
    pub types: Option<FeatureType>,
    pub worldview: Option<String>,
}

// Response Objects
#[derive(Debug, Deserialize)]
pub struct GeocodingResponse {
    pub r#type: String, // "FeatureCollection"
    pub features: Vec<Feature>,
    pub attribution: String,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    pub id: String,
    pub r#type: String, // "Feature"
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    pub r#type: String,        // "Point"
    pub coordinates: [f64; 2], // [longitude, latitude]
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub mapbox_id: String,
    pub feature_type: String,
    pub name: String,
    pub name_preferred: Option<String>,
    pub place_formatted: Option<String>,
    pub full_address: Option<String>,
    pub context: Option<Context>,
    pub coordinates: Coordinates,
    pub bbox: Option<[f64; 4]>,
    pub match_code: Option<MatchCode>,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub country: Option<ContextItem>,
    pub region: Option<ContextItem>,
    pub postcode: Option<ContextItem>,
    pub district: Option<ContextItem>,
    pub place: Option<ContextItem>,
    pub locality: Option<ContextItem>,
    pub neighborhood: Option<ContextItem>,
    pub street: Option<ContextItem>,
    pub address: Option<AddressContext>,
}

#[derive(Debug, Deserialize)]
pub struct ContextItem {
    pub mapbox_id: String,
    pub name: String,
    pub translations: Option<HashMap<String, Translation>>,
}

#[derive(Debug, Deserialize)]
pub struct Translation {
    pub language: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AddressContext {
    pub street_name: String,
    pub address_number: String,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
    pub accuracy: Option<String>,
    pub routable_points: Option<Vec<RoutablePoint>>,
}

#[derive(Debug, Deserialize)]
pub struct RoutablePoint {
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct MatchCode {
    pub confidence: String,                    // "exact", "high", "medium", "low"
    pub match_status: HashMap<String, String>, // Component type to match status mapping
}
