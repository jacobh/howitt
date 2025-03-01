use std::collections::HashMap;
use std::sync::Arc;

use crate::Context;
use howitt::ext::futures::FuturesIteratorExt;
use howitt::ext::rayon::rayon_spawn_blocking;
use howitt::models::point::delta::{Delta, DistanceDelta};
use howitt::models::point::progress::{Progress, TemporalDistanceElevationProgress};
use howitt::models::point::{Point, TemporalElevationPoint, WithDatetime};
use howitt::models::ride::{RideId, RidePointsFilter};
use howitt::models::user::UserId;
use howitt::repos::AnyhowRepo;
use howitt::services::euclidean::{geo_to_euclidean, TransformParams};
use howitt::services::stopped_time::StoppedTimeAnalyzer;
use howitt_postgresql::PostgresRepos;
use serde::Serialize;
use tokio::sync::Semaphore;

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

fn calculate_segment_metrics(idx: usize, segment_points: &[TemporalElevationPoint]) -> RideSegment {
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
    }
}

fn analyze_ride_segments(
    user_id: UserId,
    ride_id: RideId,
    ride_name: String,
    segments: Vec<Vec<TemporalElevationPoint>>,
) -> RideSegmentAnalysis {
    let mut segment_metrics = Vec::with_capacity(segments.len());
    let mut total_distance_m = 0.0;
    let mut total_elapsed_time_secs = 0;
    let mut total_stopped_time_secs = 0;
    let mut total_moving_time_secs = 0;

    for (idx, segment_points) in segments.iter().enumerate() {
        if segment_points.is_empty() {
            continue; // Skip empty segments
        }

        let segment = calculate_segment_metrics(idx, segment_points);
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

#[allow(unused_variables)]
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
    // Fetch all rides from the repository
    let all_rides = ride_repo.all().await?;
    println!("Found {} rides to analyze", all_rides.len());

    // Process these to create a HashMap for easier lookup
    let rides_by_id = all_rides
        .into_iter()
        .map(|ride| (ride.id, ride))
        .collect::<HashMap<_, _>>();

    // Extract just the ride IDs
    let ride_ids: Vec<RideId> = rides_by_id.keys().cloned().take(10).collect();

    // Batch fetch all ride points
    println!("Fetching points for all rides...");
    let all_points = ride_points_repo
        .filter_models(RidePointsFilter::Ids(ride_ids.clone()))
        .await?;

    println!("Successfully fetched points for {} rides", all_points.len());

    // Process these to create a HashMap for easier lookup
    let points_by_ride_id = all_points
        .into_iter()
        .map(|points| (points.id, points.points))
        .collect::<HashMap<_, _>>();

    // Create a semaphore to limit concurrency
    let semaphore = Arc::new(Semaphore::new(10));

    // Process each ride with its points
    let analyses = ride_ids
        .into_iter()
        .filter_map(|ride_id| {
            let ride = rides_by_id.get(&ride_id)?;
            let points = points_by_ride_id.get(&ride_id)?;

            // Skip rides with too few points
            if points.len() < 2 {
                println!(
                    "Skipping ride {} with only {} points",
                    ride_id,
                    points.len()
                );
                return None;
            }

            let ride_clone = ride.clone();
            let points_clone = points.clone();
            let semaphore_clone = semaphore.clone();

            Some(async move {
                // Acquire a permit from the semaphore
                let _permit = semaphore_clone.acquire().await.unwrap();

                // First rayon blocking call to create segments
                let segments_result = rayon_spawn_blocking(move || {
                    // Create segments (at least 250m each)
                    let segments = create_segments(points_clone, 250.0);

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
                        eprintln!("Error creating segments for ride {}: {}", ride_id, e);
                        return None;
                    }
                };

                // Second rayon blocking call to analyze segments
                let user_id = ride_clone.user_id;
                let ride_id = ride_clone.id;
                let ride_name = ride_clone.name.clone();

                Some(
                    rayon_spawn_blocking(move || {
                        // Calculate metrics for each segment
                        analyze_ride_segments(user_id, ride_id, ride_name, segments)
                    })
                    .await,
                )
            })
        })
        .collect_futures_ordered()
        .await
        .into_iter()
        .filter_map(|result| result) // Filter out the None values
        .collect::<Vec<_>>();

    if analyses.is_empty() {
        println!("No ride analyses were generated");
    } else {
        // Write output to a file instead of printing to console
        let json_content = serde_json::to_string_pretty(&analyses)?;
        std::fs::write("ride_data.json", json_content)?;
        println!(
            "Analysis results written to ride_data.json for {} rides",
            analyses.len()
        );
    }

    Ok(())
}
