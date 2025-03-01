use anyhow::anyhow;
use chrono::{DateTime, Utc};
use geojson::{GeoJson, Geometry as GeoJsonGeometry};
use howitt::ext::iter::ResultIterExt;
use howitt::models::osm::{OsmFeature, OsmFeatureFilter, OsmFeatureId};
use howitt::models::Model;
use howitt::repos::Repo;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

#[allow(dead_code)]
struct OsmFeatureRow {
    id: Uuid,
    feature_type: String,
    properties: serde_json::Value,
    geometry_type: String,
    geometry_json: String, // This will be PostGIS geometry as GeoJSON string
    created_at: DateTime<Utc>,
}

impl TryFrom<OsmFeatureRow> for OsmFeature {
    type Error = PostgresRepoError;

    fn try_from(row: OsmFeatureRow) -> Result<Self, Self::Error> {
        // Parse the GeoJSON string into a GeoJson object
        let geojson = GeoJson::from_str(&row.geometry_json)?;

        // Convert GeoJSON to geo::Geometry
        let geometry = match geojson {
            GeoJson::Geometry(geo_geom) => geo::Geometry::try_from(geo_geom)?,
            _ => {
                return Err(PostgresRepoError::Generic(anyhow!(
                    "Expected GeoJSON Geometry object"
                )))
            }
        };

        // Parse the properties JSON into a HashMap<String, String>
        let properties_map: HashMap<String, String> =
            if let serde_json::Value::Object(obj) = row.properties {
                obj.into_iter()
                    .filter_map(|(k, v)| {
                        // Convert all values to strings
                        match v {
                            serde_json::Value::String(s) => Some((k, s)),
                            serde_json::Value::Number(n) => Some((k, n.to_string())),
                            serde_json::Value::Bool(b) => Some((k, b.to_string())),
                            serde_json::Value::Null => None, // Skip null values
                            _ => Some((k, v.to_string())),   // Convert other types to string
                        }
                    })
                    .collect()
            } else {
                HashMap::new()
            };

        Ok(OsmFeature {
            id: OsmFeatureId::from(row.id),
            properties: properties_map,
            geometry,
            created_at: row.created_at,
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresOsmFeatureRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresOsmFeatureRepo {
    type Model = OsmFeature;
    type Error = PostgresRepoError;

    async fn filter_models(
        &self,
        filter: OsmFeatureFilter,
    ) -> Result<Vec<OsmFeature>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let features = match filter {
            OsmFeatureFilter::All => {
                sqlx::query_as!(
                    OsmFeatureRow,
                    r#"
                    SELECT 
                        id,
                        feature_type,
                        properties,
                        geometry_type,
                        ST_AsGeoJSON(geometry) as "geometry_json!",
                        created_at
                    FROM 
                        osm_highway_features
                    "#
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            OsmFeatureFilter::Id(id) => {
                sqlx::query_as!(
                    OsmFeatureRow,
                    r#"
                    SELECT 
                        id,
                        feature_type,
                        properties,
                        geometry_type,
                        ST_AsGeoJSON(geometry) as "geometry_json!",
                        created_at
                    FROM 
                        osm_highway_features
                    WHERE 
                        id = $1
                    "#,
                    id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            OsmFeatureFilter::NearPoint {
                point,
                max_distance_meters,
                limit,
            } => {
                // Extract coordinates from the geo::Point
                let (lon, lat) = (point.x(), point.y());

                // Use ST_DistanceSphere to find features within the specified distance
                // ST_DistanceSphere measures the spherical distance in meters
                sqlx::query_as!(
                    OsmFeatureRow,
                    r#"
                    SELECT 
                        id,
                        feature_type,
                        properties,
                        geometry_type,
                        ST_AsGeoJSON(geometry) as "geometry_json!",
                        created_at
                    FROM 
                        osm_highway_features
                    WHERE 
                        ST_DistanceSphere(
                            geometry,
                            ST_SetSRID(ST_MakePoint($1, $2), 4326)
                        ) <= $3
                    ORDER BY 
                        geometry <-> ST_SetSRID(ST_MakePoint($1, $2), 4326)
                    LIMIT $4
                    "#,
                    lon,
                    lat,
                    max_distance_meters,
                    limit.unwrap_or(100) as i64
                )
                .fetch_all(conn.as_mut())
                .await?
            }
        };

        Ok(features
            .into_iter()
            .map(OsmFeature::try_from)
            .collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<OsmFeature>, PostgresRepoError> {
        self.filter_models(OsmFeatureFilter::All).await
    }

    async fn get(&self, id: <OsmFeature as Model>::Id) -> Result<OsmFeature, PostgresRepoError> {
        let features = self.filter_models(OsmFeatureFilter::Id(id)).await?;
        features
            .into_iter()
            .next()
            .ok_or(PostgresRepoError::Generic(anyhow!("Feature not found")))
    }

    async fn put(&self, feature: OsmFeature) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        // Convert the geo::Geometry to a GeoJSON Geometry
        let geojson_geometry = GeoJsonGeometry::from(&feature.geometry);

        // Convert the GeoJSON Geometry to a string
        let geometry_json = geojson_geometry.to_string();

        // Determine the geometry type based on the geometry
        let geometry_type = match feature.geometry {
            geo::Geometry::Point(_) => "Point",
            geo::Geometry::Line(_) => "LineString",
            geo::Geometry::LineString(_) => "LineString",
            geo::Geometry::Polygon(_) => "Polygon",
            geo::Geometry::MultiPoint(_) => "MultiPoint",
            geo::Geometry::MultiLineString(_) => "MultiLineString",
            geo::Geometry::MultiPolygon(_) => "MultiPolygon",
            geo::Geometry::GeometryCollection(_) => "GeometryCollection",
            geo::Geometry::Rect(_) => "Polygon", // Rect is converted to a Polygon
            geo::Geometry::Triangle(_) => "Polygon", // Triangle is converted to a Polygon
        };

        // Convert properties to JSON
        let properties_json = serde_json::to_value(&feature.properties)?;

        sqlx::query!(
            r#"
            INSERT INTO osm_highway_features (
                id,
                feature_type,
                properties,
                geometry,
                geometry_type,
                created_at
            ) VALUES (
                $1, 
                'Feature', 
                $2, 
                ST_GeomFromGeoJSON($3), 
                $4,
                $5
            )
            ON CONFLICT (id) DO UPDATE SET
                properties = EXCLUDED.properties,
                geometry = EXCLUDED.geometry,
                geometry_type = EXCLUDED.geometry_type
            "#,
            feature.id.as_uuid(),
            properties_json,
            geometry_json,
            geometry_type,
            feature.created_at,
        )
        .execute(conn.as_mut())
        .await?;

        Ok(())
    }
}
