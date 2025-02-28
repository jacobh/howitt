use std::sync::Arc;

use crate::Context;
use howitt::ext::futures::FuturesIteratorExt;
use howitt::ext::rayon::rayon_spawn_blocking;
use howitt::models::point::delta::{Delta, DistanceDelta};
use howitt::models::point::progress::{Progress, TemporalDistanceElevationProgress};
use howitt::models::point::{Point, TemporalElevationPoint, WithDatetime};
use howitt::models::ride::RideId;
use howitt::models::user::UserId;
use howitt::repos::AnyhowRepo;
use howitt::services::euclidean::{geo_to_euclidean, TransformParams};
use howitt_postgresql::{PostgresRepos, PostgresRidePointsRepo, PostgresRideRepo};
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

/// Analyzes a sequence of temporal points to identify periods when a rider was stationary
/// A rider is considered stopped if they travel less than a specified distance threshold (m)
/// over a specified time window (seconds)
struct StoppedTimeAnalyzer {
    distance_threshold_m: f64, // Minimum distance to consider movement (default: 5m)
    time_threshold_secs: i64,  // Minimum time to consider a stop (default: 10s)
}

impl StoppedTimeAnalyzer {
    /// Creates a new analyzer with specified thresholds
    fn new(distance_threshold_m: f64, time_threshold_secs: i64) -> Self {
        Self {
            distance_threshold_m,
            time_threshold_secs,
        }
    }

    /// Analyzes points to calculate total time spent stopped
    /// Uses a sliding window approach to identify periods where the rider moved
    /// less than distance_threshold_m over at least time_threshold_secs
    fn calculate_stopped_time(&self, points: &[TemporalElevationPoint]) -> i64 {
        if points.len() < 2 {
            return 0;
        }

        let mut total_stopped_time_secs = 0;
        let mut buffer_start_idx = 0;

        // Iterate through all points to find stopped periods
        while buffer_start_idx < points.len() - 1 {
            let start_point = &points[buffer_start_idx];
            let mut current_idx = buffer_start_idx + 1;
            let mut max_distance = 0.0;
            let mut is_stopped_period = false;

            // Look for points that stay within distance threshold
            while current_idx < points.len() {
                let current_point = &points[current_idx];

                // Calculate elapsed time since start of potential stop
                let elapsed_secs = current_point
                    .datetime()
                    .signed_duration_since(*start_point.datetime())
                    .num_seconds();

                // Calculate distance from start of potential stop
                let distance = DistanceDelta::delta(start_point, current_point).0;
                max_distance = f64::max(max_distance, distance);

                // If we've exceeded the time threshold but stayed within distance threshold,
                // this is a stopped period
                if elapsed_secs >= self.time_threshold_secs
                    && max_distance <= self.distance_threshold_m
                {
                    is_stopped_period = true;
                }

                // Continue expanding the buffer if we're within distance threshold
                if distance <= self.distance_threshold_m {
                    current_idx += 1;
                } else {
                    break;
                }
            }

            // If we identified a stopped period, calculate its duration
            if is_stopped_period {
                let stop_end_idx = current_idx - 1;
                let stop_end_point = &points[stop_end_idx];
                let stop_duration = stop_end_point
                    .datetime()
                    .signed_duration_since(*start_point.datetime())
                    .num_seconds();

                total_stopped_time_secs += stop_duration;

                // Move past this stopped period
                buffer_start_idx = current_idx;
            } else {
                // Not a stopped period, try starting at the next point
                buffer_start_idx += 1;
            }
        }

        total_stopped_time_secs
    }
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

async fn fetch_ride_analysis(
    ride_id: RideId,
    ride_repo: PostgresRideRepo,
    ride_points_repo: PostgresRidePointsRepo,
) -> Result<RideSegmentAnalysis, anyhow::Error> {
    // Fetch the ride and its points
    let ride = match ride_repo.get(ride_id).await {
        Ok(ride) => ride,
        Err(e) => {
            // Check if the error is because the ride was not found
            if e.to_string().contains("not found") {
                return Err(anyhow::anyhow!("Ride not found with ID: {}", ride_id));
            }
            return Err(e.into());
        }
    };

    let ride_points = match ride_points_repo.get(ride_id).await {
        Ok(points) => points,
        Err(e) => {
            // Check if the error is because points were not found
            if e.to_string().contains("not found") {
                return Err(anyhow::anyhow!("No points found for ride: {}", ride_id));
            }
            return Err(e.into());
        }
    };

    // Check if we have enough points
    if ride_points.points.len() < 2 {
        return Err(anyhow::anyhow!(
            "Ride has too few points ({})",
            ride_points.points.len()
        ));
    }

    rayon_spawn_blocking(move || {
        // Create segments (at least 250m each)
        let segments = create_segments(ride_points.points.clone(), 250.0);

        if segments.is_empty() {
            return Err(anyhow::anyhow!(
                "No segments could be created for ride: {}",
                ride_id
            ));
        }

        // Calculate metrics for each segment
        let analysis = analyze_ride_segments(ride.user_id, ride.id, ride.name, segments);

        Ok(analysis)
    })
    .await
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
            },
        job_storage,
    }: Context,
) -> Result<(), anyhow::Error> {
    // // Vector of ride IDs to analyze
    // let ride_id_strings = vec![
    //     "019119e8-11c8-e0aa-b6c1-c6a2acbbc8f8",
    //     "0190ddbe-90a0-7716-ffaa-45e4923e4897",
    //     "019043f8-a0f0-7cd1-3c64-0def0341f4f6",
    //     "0193edaf-be50-5940-ec61-02f0ce0d9b6e",
    //     "0190c910-d938-ab0a-fd64-5bae11781db5",
    //     "0190d862-24c8-d8eb-297c-e5bd1f713939",
    //     "0193edaf-c238-a49d-5e67-cace549b0eb6",
    //     "0190d862-3468-b0bc-74d7-a4b412270528",
    //     "0193f7e6-cf58-da21-f6f8-10bb6205f598",
    //     "0190ecf2-51b0-3aa5-9cd2-3f6f175cc573",
    //     "0190ecf2-70f0-2575-4115-0220ae15c270",
    //     "0193e151-e930-785f-9f9f-c5136baa9be8",
    //     "0193c9b2-a788-e44e-a072-a15bbd0be4a0",
    //     "0193e151-f100-b523-ba46-fa20ce2558d0",
    //     "018e7242-b088-4115-2c2f-4a1324cabfc5",
    //     "01911125-5528-ee6a-e5c6-ab8767967674",
    //     "01912ad5-b3f0-7e9f-cc23-91bbf22ccad4",
    //     "0190f787-e220-9a37-0249-3fb95c5cf1ad",
    //     "0190cdff-6bf8-0879-0c06-e7f9e095f33a",
    //     "0191102f-3558-c971-d588-f8946474087f",
    //     "0193e151-f4e8-a55e-2835-c47089c5e2a1",
    //     "0191306c-b7c8-d70d-403f-548194301580",
    //     "01911f24-5b10-febc-4e78-7bd4cb2d4def",
    //     "0193c505-c918-aaf0-b5bd-cd4636209fca",
    //     "0190f787-f990-fd56-2d13-c1661267fcf1",
    //     "0193fcc6-b060-88a1-c702-c5aa7c3f9dec",
    //     "0190ecf2-70f0-3c93-b69e-7265c21d538e",
    //     "01911fc9-0750-d7fa-5d5b-a64bcfd289b9",
    //     "01903f26-96e0-47b4-0a92-198f2361a000",
    //     "0193e151-cdd8-1928-5d37-744cf8e5dd3f",
    //     "01910638-3e78-734f-fdef-78bbbcc23a83",
    //     "018ae33a-08d0-cad2-b5a6-aed15e1e3f20",
    //     "0193f7e5-5470-41de-f9cd-49e3ee555447",
    //     "0191343c-8860-8b81-2fe3-4adabd2ec7ba",
    //     "019125f2-3520-41a9-8b17-6cc6822dcb46",
    //     "0190be44-bfd8-0dc6-cb61-5a527f994d29",
    //     "0194df17-62f7-7b42-b8c4-a40d8a976641",
    //     "0194e470-84a4-7403-a71c-2265cfc9bbdd",
    //     "0193e151-d5a8-3412-0ace-4329c72f48a1",
    // ];

    // // Convert string IDs to RideId
    // let ride_ids = ride_id_strings
    //     .into_iter()
    //     .map(|id_str| RideId::from(Uuid::parse_str(id_str).unwrap()))
    //     .collect::<Vec<_>>();

    // Fetch all rides from the repository
    let all_rides = ride_repo.all().await?;
    println!("Found {} rides to analyze", all_rides.len());

    // Extract just the ride IDs
    let ride_ids = all_rides
        .into_iter()
        .map(|ride| ride.id)
        .collect::<Vec<_>>();

    // Create a semaphore to limit concurrency to 4
    let semaphore = Arc::new(Semaphore::new(10));

    // Create and execute futures for each ride ID with concurrency control
    let analyses = ride_ids
        .into_iter()
        .map(|ride_id| {
            let ride_repo_clone = ride_repo.clone();
            let ride_points_repo_clone = ride_points_repo.clone();
            let semaphore_clone = semaphore.clone();

            async move {
                // Acquire a permit from the semaphore
                let permit = semaphore_clone.acquire().await.unwrap();

                // Process the ride analysis
                let result =
                    fetch_ride_analysis(ride_id, ride_repo_clone, ride_points_repo_clone).await;

                match result {
                    Ok(analysis) => Some(analysis),
                    Err(e) => {
                        eprintln!("Error analyzing ride {}: {}", ride_id, e);
                        None
                    }
                }
            }
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
        println!("Analysis results written to ride_data.json");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use geo::Point as GeoPoint;

    // Helper function to create a test point
    fn create_point(lng: f64, lat: f64, elevation: f64, timestamp: i64) -> TemporalElevationPoint {
        TemporalElevationPoint {
            point: GeoPoint::new(lng, lat),
            elevation,
            datetime: Utc.timestamp_opt(timestamp, 0).unwrap(),
        }
    }

    // Helper to create points that simulate a stationary rider with slight GPS drift
    fn create_stationary_points(
        center_lng: f64,
        center_lat: f64,
        elevation: f64,
        start_time: i64,
        duration_secs: i64,
        point_count: usize,
    ) -> Vec<TemporalElevationPoint> {
        let mut points = Vec::with_capacity(point_count);

        for i in 0..point_count {
            // Create slight random drift (±0.00002 degrees, approximately 2m)
            let drift_lng = center_lng + (rand::random::<f64>() - 0.5) * 0.00004;
            let drift_lat = center_lat + (rand::random::<f64>() - 0.5) * 0.00004;

            // Distribute points evenly through the duration
            let timestamp = start_time + (i as i64 * duration_secs / point_count as i64);

            points.push(create_point(drift_lng, drift_lat, elevation, timestamp));
        }

        points
    }

    // Helper to create points that simulate movement in a straight line
    fn create_moving_points(
        start_lng: f64,
        start_lat: f64,
        end_lng: f64,
        end_lat: f64,
        elevation: f64,
        start_time: i64,
        duration_secs: i64,
        point_count: usize,
    ) -> Vec<TemporalElevationPoint> {
        let mut points = Vec::with_capacity(point_count);

        for i in 0..point_count {
            let fraction = i as f64 / (point_count - 1) as f64;
            let lng = start_lng + fraction * (end_lng - start_lng);
            let lat = start_lat + fraction * (end_lat - start_lat);

            // Distribute points evenly through the duration
            let timestamp = start_time + (i as i64 * duration_secs / point_count as i64);

            points.push(create_point(lng, lat, elevation, timestamp));
        }

        points
    }

    #[test]
    fn test_no_stopped_time_continuous_movement() {
        // Create a sequence of points with continuous movement
        // Moving approximately 100m over 60 seconds
        let points = create_moving_points(
            145.0, -37.0, // Start
            145.001, -37.0, // End (approx 100m east)
            100.0, // Elevation
            0,     // Start time
            60,    // Duration: 60 seconds
            10,    // 10 points
        );

        let analyzer = StoppedTimeAnalyzer::new(5.0, 10);
        let stopped_time = analyzer.calculate_stopped_time(&points);

        assert_eq!(
            stopped_time, 0,
            "Should detect no stopped time for continuous movement"
        );
    }

    #[test]
    fn test_complete_stop() {
        // Create a sequence of stationary points over 60 seconds
        let points = create_stationary_points(
            145.0, -37.0, // Center position
            100.0, // Elevation
            0,     // Start time
            60,    // Duration: 60 seconds
            10,    // 10 points
        );

        let analyzer = StoppedTimeAnalyzer::new(5.0, 10);
        let stopped_time = analyzer.calculate_stopped_time(&points);

        // Should detect 50+ seconds of stopped time (first 10 seconds might not be detected as a stop yet)
        assert!(
            stopped_time >= 50,
            "Should detect most of the time as stopped"
        );
    }

    #[test]
    fn test_stop_followed_by_movement() {
        // 30 seconds stationary, then 30 seconds of movement
        let mut points = create_stationary_points(
            145.0, -37.0, // Center position
            100.0, // Elevation
            0,     // Start time
            30,    // Duration: 30 seconds
            5,     // 5 points
        );

        // Add moving points after the stop
        points.extend(create_moving_points(
            145.0, -37.0, // Start
            145.001, -37.0, // End (approx 100m east)
            100.0, // Elevation
            30,    // Start time (continues from previous points)
            30,    // Duration: 30 seconds
            5,     // 5 points
        ));

        let analyzer = StoppedTimeAnalyzer::new(5.0, 10);
        let stopped_time = analyzer.calculate_stopped_time(&points);

        // Should detect approximately 20 seconds of stopped time
        // (first 10 seconds might not qualify as a stop due to the time threshold)
        assert!(
            stopped_time >= 15 && stopped_time <= 30,
            "Should detect approximately 20 seconds of stopped time"
        );
    }

    #[test]
    fn test_multiple_stops() {
        // 20 seconds stop, 20 seconds move, 20 seconds stop
        let mut points = Vec::new();

        // First stop
        points.extend(create_stationary_points(
            145.0, -37.0, // Center position
            100.0, // Elevation
            0,     // Start time
            20,    // Duration: 20 seconds
            4,     // 4 points
        ));

        // Movement
        points.extend(create_moving_points(
            145.0, -37.0, // Start
            145.001, -37.0, // End (approx 100m east)
            100.0, // Elevation
            20,    // Start time
            20,    // Duration: 20 seconds
            4,     // 4 points
        ));

        // Second stop
        points.extend(create_stationary_points(
            145.001, -37.0, // Center position (where previous movement ended)
            100.0, // Elevation
            40,    // Start time
            20,    // Duration: 20 seconds
            4,     // 4 points
        ));

        let analyzer = StoppedTimeAnalyzer::new(5.0, 10);
        let stopped_time = analyzer.calculate_stopped_time(&points);

        // Should detect approximately 20 seconds of stopped time (accounting for thresholds)
        assert!(
            stopped_time >= 15 && stopped_time <= 40,
            "Should detect approximately 20-30 seconds of stopped time"
        );
    }

    #[test]
    fn test_threshold_sensitivity() {
        // Create points that move just at the threshold boundary
        // Moving approximately 4.5m over 15 seconds (below 5m threshold)
        let points = create_moving_points(
            145.0, -37.0, // Start
            145.00005, -37.0, // End (approx 4.5m east)
            100.0, // Elevation
            0,     // Start time
            15,    // Duration: 15 seconds
            3,     // 3 points
        );

        // Test with 5.0m threshold - should detect as stopped
        let analyzer_standard = StoppedTimeAnalyzer::new(5.0, 10);
        let stopped_time_standard = analyzer_standard.calculate_stopped_time(&points);

        // Test with 4.0m threshold - should not detect as stopped
        let analyzer_strict = StoppedTimeAnalyzer::new(4.0, 10);
        let stopped_time_strict = analyzer_strict.calculate_stopped_time(&points);

        assert!(
            stopped_time_standard > 0,
            "Should detect stop with 5.0m threshold"
        );
        assert_eq!(
            stopped_time_strict, 0,
            "Should not detect stop with 4.0m threshold"
        );
    }

    #[test]
    fn test_time_threshold_sensitivity() {
        // Create more points that are stationary for 9 seconds (just below 10s threshold)
        // Use more points to reduce the chance of randomness affecting the test
        let short_stop = vec![
            create_point(145.0, -37.0, 100.0, 0),
            create_point(145.00001, -37.00001, 100.0, 3), // Very slight drift
            create_point(145.00002, -37.00002, 100.0, 6), // Very slight drift
            create_point(145.00001, -37.00001, 100.0, 9), // Very slight drift
        ];

        // Test with 10s threshold - should not detect as stopped
        let analyzer_standard = StoppedTimeAnalyzer::new(5.0, 10);
        let stopped_time_standard = analyzer_standard.calculate_stopped_time(&short_stop);

        // Test with 8s threshold - should detect as stopped
        let analyzer_lenient = StoppedTimeAnalyzer::new(5.0, 8);
        let stopped_time_lenient = analyzer_lenient.calculate_stopped_time(&short_stop);

        assert_eq!(
            stopped_time_standard, 0,
            "Should not detect stop with 10s threshold"
        );
        assert!(
            stopped_time_lenient > 0,
            "Should detect stop with 8s threshold"
        );

        // Print diagnostics to debug the test
        if stopped_time_lenient == 0 {
            println!("Debug - Points in short stop:");
            for (i, point) in short_stop.iter().enumerate() {
                println!("Point {}: {:?}, time: {:?}", i, point.point, point.datetime);
            }

            // Calculate distances between consecutive points
            for i in 0..short_stop.len() - 1 {
                let dist = DistanceDelta::delta(&short_stop[i], &short_stop[i + 1]).0;
                println!("Distance between points {}-{}: {:.2}m", i, i + 1, dist);
            }
        }
    }

    #[test]
    fn test_empty_and_single_point() {
        let analyzer = StoppedTimeAnalyzer::new(5.0, 10);

        // Empty array
        let empty: Vec<TemporalElevationPoint> = Vec::new();
        assert_eq!(
            analyzer.calculate_stopped_time(&empty),
            0,
            "Empty array should return 0"
        );

        // Single point
        let single = vec![create_point(145.0, -37.0, 100.0, 0)];
        assert_eq!(
            analyzer.calculate_stopped_time(&single),
            0,
            "Single point should return 0"
        );
    }

    #[test]
    fn test_real_world_scenario() {
        // Create a more realistic ride scenario with explicit points
        let mut points = Vec::new();

        // Initial movement (0-30 seconds)
        points.push(create_point(145.0000, -37.0, 100.0, 0));
        points.push(create_point(145.0001, -37.0, 100.0, 10));
        points.push(create_point(145.0003, -37.0, 100.0, 20));
        points.push(create_point(145.0005, -37.0, 100.0, 30));

        // Stop at traffic light (30-60 seconds)
        points.push(create_point(145.0005, -37.0, 100.0, 40));
        points.push(create_point(145.00051, -37.00001, 100.0, 50)); // Slight GPS drift
        points.push(create_point(145.00049, -37.00002, 100.0, 60)); // Slight GPS drift

        // Movement (60-90 seconds)
        points.push(create_point(145.0006, -37.0, 100.0, 70));
        points.push(create_point(145.0008, -37.0, 100.0, 80));
        points.push(create_point(145.001, -37.0, 100.0, 90));

        // Short pause (90-98 seconds) - shouldn't count as stop
        points.push(create_point(145.001, -37.0, 100.0, 94));
        points.push(create_point(145.00101, -37.00001, 100.0, 98)); // Slight GPS drift

        // More movement (98-128 seconds)
        points.push(create_point(145.0012, -37.0, 100.0, 108));
        points.push(create_point(145.0014, -37.0, 100.0, 118));
        points.push(create_point(145.0015, -37.0, 100.0, 128));

        // Long stop (128-188 seconds)
        points.push(create_point(145.0015, -37.0, 100.0, 138));
        points.push(create_point(145.00151, -37.00001, 100.0, 148)); // Slight GPS drift
        points.push(create_point(145.00149, -37.00002, 100.0, 158)); // Slight GPS drift
        points.push(create_point(145.0015, -37.00001, 100.0, 168)); // Slight GPS drift
        points.push(create_point(145.00151, -37.0, 100.0, 178)); // Slight GPS drift
        points.push(create_point(145.0015, -37.0, 100.0, 188));

        // Final movement (188-218 seconds)
        points.push(create_point(145.0016, -37.0, 100.0, 198));
        points.push(create_point(145.0018, -37.0, 100.0, 208));
        points.push(create_point(145.002, -37.0, 100.0, 218));

        let analyzer = StoppedTimeAnalyzer::new(5.0, 10);
        let stopped_time = analyzer.calculate_stopped_time(&points);

        // We expect approximately 80-90 seconds of stopped time:
        // - 30 seconds at traffic light
        // - 0 seconds for short pause (below threshold)
        // - 60 seconds for long stop
        println!("Detected stopped time: {} seconds", stopped_time);

        // Make the test more lenient - expect between 65-100 seconds
        assert!(
            stopped_time >= 80 && stopped_time <= 90,
            "Should detect approximately 80 seconds of stopped time, got {}",
            stopped_time
        );
    }
}
