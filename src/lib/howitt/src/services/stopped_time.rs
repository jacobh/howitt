use crate::models::point::{
    delta::{Delta, DistanceDelta},
    TemporalElevationPoint, WithDatetime,
};

/// Analyzes a sequence of temporal points to identify periods when a rider was stationary
/// A rider is considered stopped if they travel less than a specified distance threshold (m)
/// over a specified time window (seconds)
pub struct StoppedTimeAnalyzer {
    pub distance_threshold_m: f64, // Minimum distance to consider movement (default: 5m)
    pub time_threshold_secs: i64,  // Minimum time to consider a stop (default: 10s)
}

impl StoppedTimeAnalyzer {
    /// Creates a new analyzer with specified thresholds
    pub fn new(distance_threshold_m: f64, time_threshold_secs: i64) -> Self {
        Self {
            distance_threshold_m,
            time_threshold_secs,
        }
    }

    /// Analyzes points to calculate total time spent stopped
    /// Uses a sliding window approach to identify periods where the rider moved
    /// less than distance_threshold_m over at least time_threshold_secs
    pub fn calculate_stopped_time(&self, points: &[TemporalElevationPoint]) -> i64 {
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
