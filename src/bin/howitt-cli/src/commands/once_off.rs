use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::Context;
use geo::{Intersects, LineString};
use howitt::ext::futures::FuturesIteratorExt;
use howitt::ext::rayon::rayon_spawn_blocking;
use howitt::models::osm::{OsmFeature, OsmFeatureFilter, OsmFeatureId};
use howitt::models::point::delta::{Delta, DistanceDelta};
use howitt::models::point::progress::{
    DistanceElevationProgress, Progress, TemporalDistanceElevationProgress,
};
use howitt::models::point::{ElevationPoint, Point, TemporalElevationPoint, WithDatetime};
use howitt::models::ride::{RideId, RidePointsFilter};
use howitt::models::route::{RouteFilter, RouteId, RoutePointsFilter};
use howitt::models::user::UserId;
use howitt::repos::AnyhowRepo;
use howitt::services::euclidean::{geo_to_euclidean, TransformParams};
use howitt::services::simplify_points::{simplify_points_v2, DetailLevel};
use howitt::services::stopped_time::StoppedTimeAnalyzer;
use howitt_postgresql::PostgresRepos;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use tokio::sync::Semaphore;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct RideSegmentAnalysis {
    user_id: UserId,
    ride_id: RideId,
    ride_name: String,
    total_segments: usize,
    total_distance_m: f64,
    mean_elapsed_time_secs: f64,
    mean_moving_time_secs: f64,
    mean_stopped_time_secs: f64,
    mean_segment_distance_m: f64,
    segments: Vec<RideSegment>,
}

#[derive(Debug, Serialize)]
struct RideSegment {
    segment_index: usize,
    start_datetime: chrono::DateTime<chrono::Utc>,
    end_datetime: chrono::DateTime<chrono::Utc>,
    elapsed_time_secs: i64,
    stopped_time_secs: i64,
    moving_time_secs: i64,
    distance_m: f64,
    elevation_gain_m: f64,
    elevation_loss_m: f64,
    x_offset_m: f64,
    y_offset_m: f64,
    z_offset_m: f64,
    feature_properties: Option<HashMap<String, String>>,
    boundary_ids: Vec<OsmFeatureId>,
}

fn create_segments(
    points: Vec<TemporalElevationPoint>,
    min_segment_distance: f64,
) -> Vec<Vec<TemporalElevationPoint>> {
    if points.len() < 2 {
        return Vec::new();
    }

    let mut all_segments = Vec::new();
    let mut remaining_points = points;

    while !remaining_points.is_empty() {
        let start_point = &remaining_points[0];

        // Find the first point that's at least min_segment_distance away from start
        match remaining_points
            .iter()
            .position(|point| DistanceDelta::delta(start_point, point).0 >= min_segment_distance)
        {
            Some(end_idx) => {
                // Create a segment up to and including end_idx
                let current_segment = remaining_points[..=end_idx].to_vec();
                // Update remaining points starting from end_idx (including overlap)
                remaining_points = remaining_points[end_idx..].to_vec();
                // Add current segment to results
                all_segments.push(current_segment);
            }
            None => {
                // All remaining points belong to one segment
                all_segments.push(remaining_points);
                break;
            }
        }
    }

    all_segments
}

/// Rounds a floating point value to 3 decimal places
fn round_to_3dp(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn calculate_segment_metrics(
    idx: usize,
    segment_points: &[TemporalElevationPoint],
    similar_feature: Option<OsmFeature>,
    ride_boundaries: &[OsmFeature],
) -> RideSegment {
    let start_point = segment_points.first().expect("Segment should not be empty");
    let end_point = segment_points.last().expect("Segment should not be empty");

    // Calculate Euclidean coordinates
    let end_euclidean = geo_to_euclidean(TransformParams {
        origin: *start_point.as_geo_point(),
        point: *end_point.as_geo_point(),
    });

    // Calculate elevation difference (z offset)
    let z_offset_m = end_point.elevation - start_point.elevation;

    // Calculate segment-specific metrics using accumulated progress
    let progress = TemporalDistanceElevationProgress::last_from_points(segment_points.to_vec())
        .expect("Segment should have at least one point");

    // Calculate elapsed time in seconds
    let elapsed_time_secs = progress.elapsed.num_seconds();

    // Calculate stopped time using the analyzer
    let analyzer = StoppedTimeAnalyzer::new(5.0, 10);
    let stopped_time_secs = analyzer.calculate_stopped_time(segment_points);

    // Calculate moving time (elapsed time minus stopped time)
    let moving_time_secs = elapsed_time_secs - stopped_time_secs;

    let boundaries = ride_boundaries.into_iter().filter(|boundary| {
        boundary.geometry.intersects(&LineString::from_iter(
            segment_points.iter().map(|p| p.to_geo_point()),
        ))
    });

    RideSegment {
        segment_index: idx,
        start_datetime: *start_point.datetime(),
        end_datetime: *end_point.datetime(),
        elapsed_time_secs,
        stopped_time_secs,
        moving_time_secs,
        distance_m: round_to_3dp(progress.distance_m),
        elevation_gain_m: round_to_3dp(progress.elevation_gain_m),
        elevation_loss_m: round_to_3dp(progress.elevation_loss_m),
        x_offset_m: round_to_3dp(end_euclidean.x()),
        y_offset_m: round_to_3dp(end_euclidean.y()),
        z_offset_m: round_to_3dp(z_offset_m),
        feature_properties: similar_feature.map(|f| f.properties),
        boundary_ids: boundaries.map(|b| b.id()).collect(),
    }
}

fn analyze_ride_segments(
    user_id: UserId,
    ride_id: RideId,
    ride_name: String,
    segments: Vec<Vec<TemporalElevationPoint>>,
    similar_features: Vec<Option<OsmFeature>>,
    ride_boundaries: Vec<OsmFeature>,
) -> RideSegmentAnalysis {
    let mut segment_metrics = Vec::with_capacity(segments.len());
    let mut total_distance_m = 0.0;
    let mut total_elapsed_time_secs = 0;
    let mut total_stopped_time_secs = 0;
    let mut total_moving_time_secs = 0;

    for (idx, (segment_points, similar_feature)) in segments
        .iter()
        .zip(similar_features.into_iter())
        .enumerate()
    {
        if segment_points.is_empty() {
            continue; // Skip empty segments
        }

        let segment =
            calculate_segment_metrics(idx, &segment_points, similar_feature, &ride_boundaries);
        total_distance_m += segment.distance_m;
        total_elapsed_time_secs += segment.elapsed_time_secs;
        total_stopped_time_secs += segment.stopped_time_secs;
        total_moving_time_secs += segment.moving_time_secs;
        segment_metrics.push(segment);
    }

    let segment_count = segment_metrics.len();
    let mean_elapsed_time_secs = if segment_count > 0 {
        total_elapsed_time_secs as f64 / segment_count as f64
    } else {
        0.0
    };

    let mean_stopped_time_secs = if segment_count > 0 {
        total_stopped_time_secs as f64 / segment_count as f64
    } else {
        0.0
    };

    let mean_moving_time_secs = if segment_count > 0 {
        total_moving_time_secs as f64 / segment_count as f64
    } else {
        0.0
    };

    let mean_segment_distance_m = if segment_count > 0 {
        total_distance_m / segment_count as f64
    } else {
        0.0
    };

    // Round the averages to 3 decimal places for consistency
    let mean_elapsed_time_secs = round_to_3dp(mean_elapsed_time_secs);
    let mean_stopped_time_secs = round_to_3dp(mean_stopped_time_secs);
    let mean_moving_time_secs = round_to_3dp(mean_moving_time_secs);
    let mean_segment_distance_m = round_to_3dp(mean_segment_distance_m);

    RideSegmentAnalysis {
        user_id,
        ride_id,
        ride_name,
        total_segments: segments.len(),
        total_distance_m,
        mean_elapsed_time_secs,
        mean_stopped_time_secs,
        mean_moving_time_secs,
        mean_segment_distance_m,
        segments: segment_metrics,
    }
}

#[derive(Debug, Serialize)]
struct RouteSegmentAnalysis {
    route_id: RouteId,
    route_name: String,
    slug: String,
    is_forward: bool,
    total_segments: usize,
    total_distance_m: f64,
    total_elevation_gain_m: f64,
    total_elevation_loss_m: f64,
    mean_segment_distance_m: f64,
    segments: Vec<RouteSegment>,
}

#[derive(Debug, Serialize)]
struct RouteSegment {
    segment_index: usize,
    distance_m: f64,
    elevation_gain_m: f64,
    elevation_loss_m: f64,
    x_offset_m: f64,
    y_offset_m: f64,
    z_offset_m: f64,
    feature_properties: Option<HashMap<String, String>>,
}

fn create_route_segments(
    points: Vec<ElevationPoint>,
    min_segment_distance: f64,
) -> Vec<Vec<ElevationPoint>> {
    if points.len() < 2 {
        return Vec::new();
    }

    let mut all_segments = Vec::new();
    let mut remaining_points = points;

    while !remaining_points.is_empty() {
        let start_point = &remaining_points[0];

        // Find the first point that's at least min_segment_distance away from start
        match remaining_points
            .iter()
            .position(|point| DistanceDelta::delta(start_point, point).0 >= min_segment_distance)
        {
            Some(end_idx) => {
                // Create a segment up to and including end_idx
                let current_segment = remaining_points[..=end_idx].to_vec();
                // Update remaining points starting from end_idx (including overlap)
                remaining_points = remaining_points[end_idx..].to_vec();
                // Add current segment to results
                all_segments.push(current_segment);
            }
            None => {
                // All remaining points belong to one segment
                all_segments.push(remaining_points);
                break;
            }
        }
    }

    all_segments
}

fn calculate_route_segment_metrics(
    idx: usize,
    segment_points: &[ElevationPoint],
    similar_feature: Option<OsmFeature>,
) -> RouteSegment {
    let start_point = segment_points.first().expect("Segment should not be empty");
    let end_point = segment_points.last().expect("Segment should not be empty");

    // Calculate Euclidean coordinates
    let end_euclidean = geo_to_euclidean(TransformParams {
        origin: *start_point.as_geo_point(),
        point: *end_point.as_geo_point(),
    });

    // Calculate elevation difference (z offset)
    let z_offset_m = end_point.elevation - start_point.elevation;

    // Calculate segment-specific metrics using accumulated progress
    let progress = DistanceElevationProgress::last_from_points(segment_points.to_vec())
        .expect("Segment should have at least one point");

    RouteSegment {
        segment_index: idx,
        distance_m: round_to_3dp(progress.distance_m),
        elevation_gain_m: round_to_3dp(progress.elevation_gain_m),
        elevation_loss_m: round_to_3dp(progress.elevation_loss_m),
        x_offset_m: round_to_3dp(end_euclidean.x()),
        y_offset_m: round_to_3dp(end_euclidean.y()),
        z_offset_m: round_to_3dp(z_offset_m),
        feature_properties: similar_feature.map(|f| f.properties),
    }
}

fn analyze_route_segments(
    route_id: RouteId,
    route_name: String,
    slug: String,
    is_forward: bool,
    segments: Vec<Vec<ElevationPoint>>,
    similar_features: Vec<Option<OsmFeature>>,
) -> RouteSegmentAnalysis {
    let mut segment_metrics = Vec::with_capacity(segments.len());
    let mut total_distance_m = 0.0;
    let mut total_elevation_gain_m = 0.0;
    let mut total_elevation_loss_m = 0.0;

    for (idx, (segment_points, similar_feature)) in segments
        .iter()
        .zip(similar_features.into_iter())
        .enumerate()
    {
        if segment_points.is_empty() {
            continue; // Skip empty segments
        }

        let segment = calculate_route_segment_metrics(idx, &segment_points, similar_feature);
        total_distance_m += segment.distance_m;
        total_elevation_gain_m += segment.elevation_gain_m;
        total_elevation_loss_m += segment.elevation_loss_m;
        segment_metrics.push(segment);
    }

    let segment_count = segment_metrics.len();
    let mean_segment_distance_m = if segment_count > 0 {
        total_distance_m / segment_count as f64
    } else {
        0.0
    };

    // Round the average to 3 decimal places for consistency
    let mean_segment_distance_m = round_to_3dp(mean_segment_distance_m);
    // Round the totals for consistency
    let total_elevation_gain_m = round_to_3dp(total_elevation_gain_m);
    let total_elevation_loss_m = round_to_3dp(total_elevation_loss_m);

    RouteSegmentAnalysis {
        route_id,
        route_name,
        slug,
        is_forward,
        total_segments: segments.len(),
        total_distance_m,
        total_elevation_gain_m,
        total_elevation_loss_m,
        mean_segment_distance_m,
        segments: segment_metrics,
    }
}

async fn process_routes(context: &Context, route_ids: Vec<RouteId>) -> Result<(), anyhow::Error> {
    // Fetch all routes from the repository
    println!("Fetching routes...");
    let route_repo = &context.repos.route_repo;
    let route_points_repo = &context.repos.route_points_repo;
    let osm_feature_repo = &context.repos.osm_feature_repo;

    let all_routes = route_repo.all().await?;
    println!("Found {} routes in database", all_routes.len());

    // Process these to create a HashMap for easier lookup
    let routes_by_id = all_routes
        .into_iter()
        .map(|route| (route.id, route))
        .collect::<HashMap<_, _>>();

    // Create a progress bar for data fetching
    let fetch_pb = ProgressBar::new_spinner();
    fetch_pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    fetch_pb.set_message("Fetching points for selected routes...");
    fetch_pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // Batch fetch all route points
    let all_points = route_points_repo
        .filter_models(RoutePointsFilter::Ids(route_ids.clone()))
        .await?;

    fetch_pb.finish_with_message(format!(
        "Successfully fetched points for {} routes",
        all_points.len()
    ));

    // Process these to create a HashMap for easier lookup
    let points_by_route_id = all_points
        .into_iter()
        .map(|points| (points.id, points.points))
        .collect::<HashMap<_, _>>();

    // Create a semaphore to limit concurrency
    let semaphore = Arc::new(Semaphore::new(10));

    // Create a counter for completed routes
    let completed_routes = Arc::new(AtomicUsize::new(0));

    // Create a progress bar for route processing
    let process_pb = ProgressBar::new(route_ids.len() as u64);
    process_pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("##-"),
    );
    process_pb.set_message("Processing routes...");

    // Prepare a vector for valid route IDs to be processed
    let valid_route_ids: Vec<_> = route_ids
        .into_iter()
        .filter(|route_id| {
            let route_exists = routes_by_id.contains_key(route_id);
            let has_points = match points_by_route_id.get(route_id) {
                Some(points) => points.len() >= 2,
                None => false,
            };

            if !route_exists {
                process_pb.println(format!("Route {} not found in database", route_id));
                process_pb.inc(1);
                return false;
            }

            if !has_points {
                process_pb.println(format!(
                    "Skipping route {} with insufficient points",
                    route_id
                ));
                process_pb.inc(1);
                return false;
            }

            true
        })
        .collect();

    // Update total count to reflect only valid routes
    process_pb.set_length(valid_route_ids.len() as u64);

    // Process each valid route with its points
    let process_pb_clone = process_pb.clone();
    let completed_routes_clone = completed_routes.clone();

    // We're processing each route twice, so double the progress bar length
    process_pb.set_length(valid_route_ids.len() as u64 * 2);

    let analyses = valid_route_ids
        .into_iter()
        .flat_map(|route_id| {
            // For each route, create two processing tasks - forward and reverse
            let route = routes_by_id.get(&route_id).unwrap().clone();
            let points = points_by_route_id.get(&route_id).unwrap().clone();

            // Create two iterations - one forward, one reverse
            vec![
                (route_id, route.clone(), points.clone(), true), // Forward
                (
                    route_id,
                    route.clone(),
                    points.clone().into_iter().rev().collect(),
                    false,
                ), // Reverse
            ]
        })
        .map(|(route_id, route, points, is_forward)| {
            let semaphore_clone = semaphore.clone();
            let osm_feature_repo = osm_feature_repo.clone();
            let pb_clone = process_pb_clone.clone();
            let completed = completed_routes_clone.clone();

            async move {
                // Acquire a permit from the semaphore
                let _permit = semaphore_clone.acquire().await.unwrap();

                // Update progress message with current route and direction
                let direction_text = if is_forward { "forward" } else { "reverse" };
                pb_clone.set_message(format!(
                    "Processing route {} ({}) {}",
                    route_id, route.name, direction_text
                ));

                // First rayon blocking call to create segments
                let segments_result = rayon_spawn_blocking(move || {
                    // Create segments (at least 250m each)
                    let segments = create_route_segments(points, 250.0);

                    if segments.is_empty() {
                        return Err(anyhow::anyhow!(
                            "No segments could be created for route: {} ({})",
                            route_id,
                            direction_text
                        ));
                    }

                    Ok(segments)
                })
                .await;

                let segments = match segments_result {
                    Ok(segments) => segments,
                    Err(e) => {
                        pb_clone.println(format!(
                            "Error creating segments for route {} ({}): {}",
                            route_id, direction_text, e
                        ));
                        pb_clone.inc(1);
                        completed.fetch_add(1, Ordering::SeqCst);
                        return None;
                    }
                };

                let segments2 = segments.clone();

                let simplified_segments = rayon_spawn_blocking(move || {
                    segments2
                        .into_iter()
                        .map(|segment| simplify_points_v2(segment, DetailLevel::Low))
                        .collect_vec()
                })
                .await;

                pb_clone.set_message(format!(
                    "Finding similar features for route {} ({})",
                    route_id, direction_text
                ));
                let similar_features = simplified_segments
                    .into_iter()
                    .map(|segment| async {
                        osm_feature_repo
                            .find_model(OsmFeatureFilter::SimilarToGeometry {
                                geometry: geo::Geometry::LineString(LineString::from_iter(
                                    segment.into_iter().map(|p| p.to_geo_point()),
                                )),
                                limit: Some(1),
                                is_highway: true,
                            })
                            .await
                            .ok()
                            .flatten()
                    })
                    .collect_futures_ordered()
                    .await;

                // Second rayon blocking call to analyze segments
                let route_id = route.id;
                let route_name = route.name.clone();
                let slug = route.slug.clone();

                pb_clone.set_message(format!(
                    "Analyzing segments for route {} ({})",
                    route_id, direction_text
                ));
                let result = rayon_spawn_blocking(move || {
                    // Calculate metrics for each segment
                    analyze_route_segments(
                        route_id,
                        route_name,
                        slug,
                        is_forward, // Pass the direction flag
                        segments,
                        similar_features,
                    )
                })
                .await;

                // Update progress
                pb_clone.inc(1);
                completed.fetch_add(1, Ordering::SeqCst);

                // Show the total progress
                let done = completed.load(Ordering::SeqCst);
                pb_clone.set_message(format!(
                    "Completed {}/{} route analyses",
                    done,
                    pb_clone.length().unwrap()
                ));

                Some(result)
            }
        })
        .collect_futures_ordered()
        .await
        .into_iter()
        .filter_map(|result| result) // Filter out the None values
        .collect::<Vec<_>>();

    // Finalize the progress bar
    process_pb.finish_with_message(format!("Completed processing {} routes", analyses.len()));

    if analyses.is_empty() {
        println!("No route analyses were generated");
    } else {
        // Create a progress bar for writing output
        let write_pb = ProgressBar::new_spinner();
        write_pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        write_pb.set_message("Writing results to file...");
        write_pb.enable_steady_tick(std::time::Duration::from_millis(100));

        // Write output to a file
        let json_content = serde_json::to_string_pretty(&analyses)?;
        std::fs::write("route_data.json", json_content)?;

        write_pb.finish_with_message(format!(
            "Analysis results written to route_data.json for {} routes",
            analyses.len()
        ));
    }

    Ok(())
}

#[allow(unused_variables)]
pub async fn handle(context: Context) -> Result<(), anyhow::Error> {
    let Context {
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
    } = &context;

    // // Define route IDs to analyze
    // const ROUTE_ID_STRS: &[&str] = &[
    //     "018b4c6d-b4a0-fb6e-02b3-f5688562ab67",
    //     "018afe4c-9390-ff2a-74d3-4d1797cf6092",
    //     "018b4c81-7f08-9ea1-40cf-b93993e0d187",
    //     "018b4c72-6ba8-79fc-ab63-33ed9b83b8fd",
    // ];

    // // // Parse route IDs
    // // let route_ids = ROUTE_ID_STRS
    // //     .iter()
    // //     .map(|id_str| RouteId::from(Uuid::parse_str(id_str).unwrap()))
    // //     .collect::<Vec<_>>();

    // // println!("Processing {} routes", route_ids.len());
    // let route_ids = route_repo
    //     .filter_models(RouteFilter::Starred)
    //     .await?
    //     .into_iter()
    //     .map(|route| route.id)
    //     .collect_vec();

    // // Process routes first
    // process_routes(&context, route_ids).await?;

    // return Ok(());

    // Fetch all rides from the repository
    println!("Fetching all rides...");
    let all_rides = ride_repo.all().await?;
    println!("Found {} rides to analyze", all_rides.len());

    // Process these to create a HashMap for easier lookup
    let rides_by_id = all_rides
        .into_iter()
        .map(|ride| (ride.id, ride))
        // .take(10)
        .collect::<HashMap<_, _>>();

    // Extract just the ride IDs
    let ride_ids: Vec<RideId> = rides_by_id.keys().cloned().collect();
    // ride_ids.shuffle(&mut thread_rng());

    // // Take only 100 rides for analysis
    // let total_to_process = 10;
    // let ride_ids = ride_ids
    //     .into_iter()
    //     .take(total_to_process)
    //     .collect::<Vec<_>>();

    // Create a progress bar for data fetching
    let fetch_pb = ProgressBar::new_spinner();
    fetch_pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    fetch_pb.set_message("Fetching points for all rides...");
    fetch_pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // Batch fetch all ride points
    let all_points = ride_points_repo
        .filter_models(RidePointsFilter::Ids(ride_ids.clone()))
        .await?;

    fetch_pb.finish_with_message(format!(
        "Successfully fetched points for {} rides",
        all_points.len()
    ));

    // Process these to create a HashMap for easier lookup
    let points_by_ride_id = all_points
        .into_iter()
        .map(|points| (points.id, points.points))
        .collect::<HashMap<_, _>>();

    // Create a semaphore to limit concurrency
    let semaphore = Arc::new(Semaphore::new(10));

    // Create a counter for completed rides
    let completed_rides = Arc::new(AtomicUsize::new(0));

    // Create a progress bar for ride processing
    let process_pb = ProgressBar::new(ride_ids.len() as u64);
    process_pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("##-"),
    );
    process_pb.set_message("Processing rides...");

    // Prepare a vector for valid ride IDs to be processed
    let valid_ride_ids: Vec<_> = ride_ids
        .into_iter()
        .filter(|ride_id| {
            let has_points = match points_by_ride_id.get(ride_id) {
                Some(points) => points.len() >= 2,
                None => false,
            };

            if !has_points {
                process_pb.println(format!(
                    "Skipping ride {} with insufficient points",
                    ride_id
                ));
                process_pb.inc(1); // Update progress even for skipped rides
            }

            has_points
        })
        .collect();

    // Update total count to reflect only valid rides
    process_pb.set_length(valid_ride_ids.len() as u64);

    // Process each valid ride with its points
    let process_pb_clone = process_pb.clone();
    let completed_rides_clone = completed_rides.clone();

    let analyses = valid_ride_ids
        .into_iter()
        .map(|ride_id| {
            let ride = rides_by_id.get(&ride_id).unwrap().clone();
            let points = points_by_ride_id.get(&ride_id).unwrap().clone();
            let semaphore_clone = semaphore.clone();
            let osm_feature_repo = osm_feature_repo.clone();
            let pb_clone = process_pb_clone.clone();
            let completed = completed_rides_clone.clone();

            async move {
                // Acquire a permit from the semaphore
                let _permit = semaphore_clone.acquire().await.unwrap();

                // Update progress message with current ride
                pb_clone.set_message(format!("Processing ride {} ({})", ride_id, ride.name));

                // First rayon blocking call to create segments
                let segments_result = rayon_spawn_blocking(move || {
                    // Create segments (at least 250m each)
                    let segments = create_segments(points, 250.0);

                    if segments.is_empty() {
                        return Err(anyhow::anyhow!(
                            "No segments could be created for ride: {}",
                            ride_id
                        ));
                    }

                    Ok(segments)
                })
                .await;

                let segments = match segments_result {
                    Ok(segments) => segments,
                    Err(e) => {
                        pb_clone.println(format!(
                            "Error creating segments for ride {}: {}",
                            ride_id, e
                        ));
                        pb_clone.inc(1);
                        completed.fetch_add(1, Ordering::SeqCst);
                        return None;
                    }
                };

                let segments2 = segments.clone();

                let simplified_segments = rayon_spawn_blocking(move || {
                    segments2
                        .into_iter()
                        .map(|segment| simplify_points_v2(segment, DetailLevel::Low))
                        .collect_vec()
                })
                .await;

                pb_clone.set_message(format!("Finding similar features for ride {}", ride_id));
                let similar_features = simplified_segments
                    .into_iter()
                    .map(|segment| async {
                        osm_feature_repo
                            .find_model(OsmFeatureFilter::SimilarToGeometry {
                                geometry: geo::Geometry::LineString(LineString::from_iter(
                                    segment.into_iter().map(|p| p.to_geo_point()),
                                )),
                                is_highway: true,
                                limit: Some(1),
                            })
                            .await
                            .ok()
                            .flatten()
                    })
                    .collect_futures_ordered()
                    .await;

                let boundaries = osm_feature_repo
                    .filter_models(OsmFeatureFilter::IntersectsRide { ride_id })
                    .await
                    .unwrap_or_default();

                // Second rayon blocking call to analyze segments
                let user_id = ride.user_id;
                let ride_id = ride.id;
                let ride_name = ride.name.clone();

                pb_clone.set_message(format!("Analyzing segments for ride {}", ride_id));
                let result = rayon_spawn_blocking(move || {
                    // Calculate metrics for each segment
                    analyze_ride_segments(
                        user_id,
                        ride_id,
                        ride_name,
                        segments,
                        similar_features,
                        boundaries,
                    )
                })
                .await;

                // Update progress
                pb_clone.inc(1);
                completed.fetch_add(1, Ordering::SeqCst);

                // Show the total progress
                let done = completed.load(Ordering::SeqCst);
                pb_clone.set_message(format!(
                    "Completed {}/{} rides",
                    done,
                    pb_clone.length().unwrap()
                ));

                Some(result)
            }
        })
        .collect_futures_ordered()
        .await
        .into_iter()
        .filter_map(|result| result) // Filter out the None values
        .collect::<Vec<_>>();

    // Finalize the progress bar
    process_pb.finish_with_message(format!("Completed processing {} rides", analyses.len()));

    if analyses.is_empty() {
        println!("No ride analyses were generated");
    } else {
        // Create a progress bar for writing output
        let write_pb = ProgressBar::new_spinner();
        write_pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        write_pb.set_message("Writing results to file...");
        write_pb.enable_steady_tick(std::time::Duration::from_millis(100));

        // Write output to a file instead of printing to console
        let json_content = serde_json::to_string_pretty(&analyses)?;
        std::fs::write("ride_data.json", json_content)?;

        write_pb.finish_with_message(format!(
            "Analysis results written to ride_data.json for {} rides",
            analyses.len()
        ));
    }

    Ok(())
}
