// src/bin/howitt-cli/src/commands/once_off.rs
use geo::Geometry;
use geojson::{GeoJson, Geometry as GeoJsonGeometry};
use howitt::models::osm::{OsmFeatureFilter, OsmFeatureId};
use howitt::repos::Repo;
use howitt_postgresql::PostgresRepos;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

use crate::Context;

pub async fn handle(
    Context {
        postgres_client,
        repos:
            PostgresRepos {
                user_repo,
                ride_repo,
                ride_points_repo,
                trip_repo,
                media_repo,
                route_repo,
                route_points_repo,
                point_of_interest_repo,
                osm_feature_repo,
            },
        job_storage,
    }: Context,
) -> Result<(), anyhow::Error> {
    // First test: retrieve by ID
    let feature_uuid_str = "38338862-3ec5-49db-94d2-6c33ff352eef";

    println!(
        "Testing OSM feature repo with hardcoded ID: {}",
        feature_uuid_str
    );

    let feature_uuid = Uuid::parse_str(feature_uuid_str)?;
    let feature_id = OsmFeatureId::from(feature_uuid);

    // Retrieve the feature using the Id filter
    println!("Retrieving feature by ID...");
    let features = osm_feature_repo
        .filter_models(OsmFeatureFilter::Id(feature_id))
        .await?;

    // Check if we got the feature back
    if let Some(feature) = features.first() {
        println!("Successfully retrieved feature by ID: {}", feature.id);
        println!("Feature properties:");
        for (key, value) in &feature.properties {
            println!("  {}: {}", key, value);
        }
        println!("Geometry type: {:?}", feature.geometry);
    } else {
        println!("No feature found with ID: {}", feature_uuid_str);
    }

    // Test the NearPoint filter with the provided test point
    println!("\n=== Testing NearPoint filter ===");

    // Create a geo::Point from the provided GeoJSON coordinates
    let test_point = geo::Point::new(145.3361542253511, -37.58254209146193);
    println!("Test point: {:?}", test_point);

    // Set search parameters
    let max_distance_meters = 1000.0; // 1km radius
    let limit = Some(5); // Get up to 5 nearest features

    println!(
        "Searching for features within {} meters...",
        max_distance_meters
    );

    // Execute the NearPoint filter
    let nearby_features = osm_feature_repo
        .filter_models(OsmFeatureFilter::NearPoint {
            point: test_point,
            max_distance_meters,
            limit,
        })
        .await?;

    // Display results
    println!("Found {} nearby features:", nearby_features.len());

    for (i, feature) in nearby_features.iter().enumerate() {
        println!("\nFeature #{}", i + 1);
        println!("ID: {}", feature.id);

        // Print highway property if it exists
        if let Some(highway_type) = feature.properties.get("highway") {
            println!("Highway type: {}", highway_type);
        }

        // Print name if it exists
        if let Some(name) = feature.properties.get("name") {
            println!("Name: {}", name);
        }

        // Print all properties
        println!("All properties:");
        for (key, value) in &feature.properties {
            println!("  {}: {}", key, value);
        }

        println!("Geometry type: {:?}", feature.geometry);
    }

    // Test the SimilarToGeometry filter
    println!("\n=== Testing SimilarToGeometry filter ===");

    // Sample LineString geometry using GeoJSON format
    let geojson_value = json!({
        "type": "LineString",
        "coordinates": [
            [145.01749841634842, -37.78962722016847],
            [145.0198619754048, -37.79207038155238],
            [145.01995693983048, -37.79265406002954],
            [145.0194610144933, -37.793112661313],
            [145.0178650845491, -37.79292296690176],
            [145.0166490123126, -37.79288336001298],
            [145.01451231272216, -37.79232053003594],
            [145.01429072906137, -37.79368799344229],
            [145.01474444798606, -37.79421329277897]
        ]
    });

    // Parse GeoJSON string into a GeoJson Geometry
    let geojson = GeoJson::try_from(geojson_value)?;
    let geo_geometry = match geojson {
        GeoJson::Geometry(geo_geom) => geo::Geometry::try_from(geo_geom)?,
        _ => return Err(anyhow::anyhow!("Expected GeoJSON Geometry object")),
    };

    println!("Test geometry: {:?}", geo_geometry);

    // Set limit parameter
    let limit = Some(5); // Get up to 5 most similar features

    println!("Searching for similar features...");

    // Execute the SimilarToGeometry filter
    let similar_features = osm_feature_repo
        .filter_models(OsmFeatureFilter::SimilarToGeometry {
            geometry: geo_geometry,
            limit,
        })
        .await?;

    // Display results
    println!("Found {} similar features:", similar_features.len());

    for (i, feature) in similar_features.iter().enumerate() {
        println!("\nFeature #{}", i + 1);
        println!("ID: {}", feature.id);

        // Print highway property if it exists
        if let Some(highway_type) = feature.properties.get("highway") {
            println!("Highway type: {}", highway_type);
        }

        // Print name if it exists
        if let Some(name) = feature.properties.get("name") {
            println!("Name: {}", name);
        }

        // Print all properties
        println!("All properties:");
        for (key, value) in &feature.properties {
            println!("  {}: {}", key, value);
        }

        println!("Geometry type: {:?}", feature.geometry);
    }

    println!("\nOSM feature repo test completed");

    Ok(())
}
