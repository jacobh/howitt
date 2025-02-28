use crate::Context;
use howitt::models::point::delta::{Delta, DistanceDelta};
use howitt::models::point::progress::{Progress, TemporalDistanceElevationProgress};
use howitt::models::point::{Point, TemporalElevationPoint, WithDatetime};
use howitt::models::ride::RideId;
use howitt::repos::AnyhowRepo;
use howitt::services::euclidean::{geo_to_euclidean, TransformParams};
use howitt_postgresql::PostgresRepos;
use itertools::Itertools;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct RideSegmentAnalysis {
    ride_id: String,
    ride_name: String,
    total_segments: usize,
    total_distance_m: f64,
    mean_elapsed_time_secs: f64,
    mean_segment_distance_m: f64,
    segments: Vec<RideSegment>,
}

#[derive(Debug, Serialize)]
struct RideSegment {
    segment_index: usize,
    start_datetime: chrono::DateTime<chrono::Utc>,
    end_datetime: chrono::DateTime<chrono::Utc>,
    elapsed_time_secs: i64,
    distance_m: f64,
    elevation_gain_m: f64,
    elevation_loss_m: f64,
    x_offset_m: f64,
    y_offset_m: f64,
}

fn create_segments(
    points: Vec<TemporalElevationPoint>,
    min_segment_distance: f64,
) -> Box<dyn Iterator<Item = Vec<TemporalElevationPoint>> + 'static> {
    if points.len() < 2 {
        return Box::new(std::iter::empty());
    }

    let start_point = &points[0];

    // Find the first point that's at least min_segment_distance away from start
    match points
        .iter()
        .position(|point| DistanceDelta::delta(start_point, point).0 >= min_segment_distance)
    {
        Some(end_idx) => {
            // Create a segment up to and including end_idx
            let current_segment = points[..=end_idx].to_vec();
            // Create remaining points starting from end_idx (including overlap)
            let remaining = points[end_idx..].to_vec();

            // Chain current segment with recursive results
            Box::new(
                std::iter::once(current_segment)
                    .chain(create_segments(remaining, min_segment_distance)),
            )
        }
        None => {
            // All points belong to one segment
            Box::new(std::iter::once(points))
        }
    }
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

    // Calculate segment-specific metrics using accumulated progress
    let progress = TemporalDistanceElevationProgress::last_from_points(segment_points.to_vec())
        .expect("Segment should have at least one point");

    // Calculate elapsed time in seconds
    let elapsed_time_secs = progress.elapsed.num_seconds();

    RideSegment {
        segment_index: idx,
        start_datetime: *start_point.datetime(),
        end_datetime: *end_point.datetime(),
        elapsed_time_secs,
        distance_m: round_to_3dp(progress.distance_m),
        elevation_gain_m: round_to_3dp(progress.elevation_gain_m),
        elevation_loss_m: round_to_3dp(progress.elevation_loss_m),
        x_offset_m: round_to_3dp(end_euclidean.x()),
        y_offset_m: round_to_3dp(end_euclidean.y()),
    }
}

fn analyze_ride_segments(
    ride_id: RideId,
    ride_name: String,
    segments: Vec<Vec<TemporalElevationPoint>>,
) -> RideSegmentAnalysis {
    let mut segment_metrics = Vec::with_capacity(segments.len());
    let mut total_distance_m = 0.0;
    let mut total_elapsed_time_secs = 0;

    for (idx, segment_points) in segments.iter().enumerate() {
        if segment_points.is_empty() {
            continue; // Skip empty segments
        }

        let segment = calculate_segment_metrics(idx, segment_points);
        total_distance_m += segment.distance_m;
        total_elapsed_time_secs += segment.elapsed_time_secs;
        segment_metrics.push(segment);
    }

    let segment_count = segment_metrics.len();
    let mean_elapsed_time_secs = if segment_count > 0 {
        total_elapsed_time_secs as f64 / segment_count as f64
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
    let mean_segment_distance_m = round_to_3dp(mean_segment_distance_m);

    RideSegmentAnalysis {
        ride_id: ride_id.to_string(),
        ride_name,
        total_segments: segments.len(),
        total_distance_m,
        mean_elapsed_time_secs,
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
            },
        job_storage,
    }: Context,
) -> Result<(), anyhow::Error> {
    let ride_id_str = "0194e470-84a4-7403-a71c-2265cfc9bbdd";
    let ride_id = match Uuid::parse_str(ride_id_str) {
        Ok(uuid) => RideId::from(uuid),
        Err(_) => {
            eprintln!("Invalid ride ID format: {}", ride_id_str);
            return Ok(());
        }
    };

    // Fetch the ride and its points
    let ride = match ride_repo.get(ride_id).await {
        Ok(ride) => ride,
        Err(e) => {
            // Check if the error is because the ride was not found
            if e.to_string().contains("not found") {
                println!("Ride not found with ID: {}", ride_id);
                return Ok(());
            }
            eprintln!("Error fetching ride: {}", e);
            return Err(e.into());
        }
    };

    let ride_points = match ride_points_repo.get(ride_id).await {
        Ok(points) => points,
        Err(e) => {
            // Check if the error is because points were not found
            if e.to_string().contains("not found") {
                println!("No points found for ride: {}", ride_id);
                return Ok(());
            }
            eprintln!("Error fetching ride points: {}", e);
            return Err(e.into());
        }
    };

    // Check if we have enough points
    if ride_points.points.len() < 2 {
        println!("Ride has too few points ({})", ride_points.points.len());
        return Ok(());
    }

    // Create segments (at least 250m each)
    let segments = create_segments(ride_points.points.clone(), 250.0).collect_vec();

    if segments.is_empty() {
        println!("No segments could be created");
        return Ok(());
    }

    // Calculate metrics for each segment
    let analysis = analyze_ride_segments(ride.id, ride.name, segments);

    // Output as JSON
    println!("{}", serde_json::to_string_pretty(&analysis)?);

    Ok(())
}
