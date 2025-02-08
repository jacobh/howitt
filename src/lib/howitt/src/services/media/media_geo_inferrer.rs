use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

use crate::{
    models::{
        filters::TemporalFilter,
        media::Media,
        point::Point,
        ride::{Ride, RideFilter},
    },
    repos::{MediaRepo, RidePointsRepo, RideRepo},
};

#[derive(Error, Debug)]
pub enum MediaGeoInferrerError {
    #[error("media has no captured_at timestamp")]
    NoCapturedAt,
}

pub struct MediaGeoInferrer {
    media_repo: MediaRepo,
    ride_repo: RideRepo,
    ride_points_repo: RidePointsRepo,
}

#[derive(Debug)]
pub struct InferredLocation {
    pub ride: Ride,
    pub point: geo::Point,
}

impl MediaGeoInferrer {
    pub fn new(
        media_repo: MediaRepo,
        ride_repo: RideRepo,
        ride_points_repo: RidePointsRepo,
    ) -> Self {
        Self {
            media_repo,
            ride_repo,
            ride_points_repo,
        }
    }

    /// Find a matching ride for a given timestamp by looking for:
    /// 1. An exact match where the timestamp falls within a ride's duration
    /// 2. If no exact match, find the most recently finished ride before the timestamp
    pub fn find_matching_ride(
        captured_at: DateTime<Utc>,
        candidate_rides: Vec<Ride>,
    ) -> Option<Ride> {
        // First try to find a ride where the timestamp falls between start and finish
        let exact_matching_ride = candidate_rides
            .iter()
            .find(|ride| captured_at >= ride.started_at && captured_at <= ride.finished_at)
            .cloned();

        if let Some(ride) = exact_matching_ride {
            Some(ride)
        } else {
            // If no exact match, find the ride that finished most recently before the timestamp
            candidate_rides
                .into_iter()
                .filter(|ride| ride.finished_at <= captured_at)
                .max_by_key(|ride| ride.finished_at)
        }
    }

    pub async fn infer_ride_and_point(
        &self,
        media: &Media,
    ) -> Result<Option<InferredLocation>, anyhow::Error> {
        let captured_at = match media.captured_at {
            Some(captured_at) => captured_at,
            None => return Err(MediaGeoInferrerError::NoCapturedAt.into()),
        };

        // Find rides within a 2 day window before captured_at
        let two_days = Duration::days(2);
        let after = captured_at - two_days;

        let candidate_rides = self
            .ride_repo
            .filter_models(RideFilter::ForUser {
                user_id: media.user_id,
                started_at: Some(TemporalFilter::After {
                    after,
                    first: Some(10),
                }),
            })
            .await?;

        let matching_ride = Self::find_matching_ride(captured_at, candidate_rides);

        let matching_ride = match matching_ride {
            Some(ride) => ride,
            None => return Ok(None),
        };

        // Get the ride points
        let ride_points = self.ride_points_repo.get(matching_ride.id).await?;

        // Find the point closest in time to captured_at
        let closest_point = ride_points
            .points
            .into_iter()
            .min_by_key(|point| {
                let duration = if point.datetime > captured_at {
                    point.datetime - captured_at
                } else {
                    captured_at - point.datetime
                };
                duration
            })
            .map(|point| *point.as_geo_point());

        match closest_point {
            Some(point) => Ok(Some(InferredLocation {
                ride: matching_ride,
                point,
            })),
            None => Ok(None),
        }
    }

    pub async fn infer_ride_and_point_and_save(&self, media: &Media) -> Result<(), anyhow::Error> {
        if let Some(inferred) = self.infer_ride_and_point(media).await? {
            let mut updated_media = media.clone();
            updated_media.point = Some(inferred.point);
            self.media_repo.put(updated_media).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ride::RideId, user::UserId};

    use super::*;
    use chrono::{TimeZone, Utc};

    fn create_test_ride(id: u32, started_at: DateTime<Utc>, finished_at: DateTime<Utc>) -> Ride {
        Ride {
            id: RideId::new(),
            name: format!("Test Ride {}", id),
            user_id: UserId::new(),
            distance: 0.0,
            started_at,
            finished_at,
            external_ref: None,
        }
    }

    #[test]
    fn test_find_matching_ride_exact_match() {
        let captured_at = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();

        let rides = vec![
            create_test_ride(
                1,
                Utc.with_ymd_and_hms(2023, 1, 1, 11, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2023, 1, 1, 13, 0, 0).unwrap(),
            ),
            create_test_ride(
                2,
                Utc.with_ymd_and_hms(2023, 1, 1, 14, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2023, 1, 1, 15, 0, 0).unwrap(),
            ),
        ];

        let result = MediaGeoInferrer::find_matching_ride(captured_at, rides);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Test Ride 1");
    }

    #[test]
    fn test_find_matching_ride_most_recent_before() {
        let captured_at = Utc.with_ymd_and_hms(2023, 1, 1, 16, 0, 0).unwrap();

        let rides = vec![
            create_test_ride(
                1,
                Utc.with_ymd_and_hms(2023, 1, 1, 11, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap(),
            ),
            create_test_ride(
                2,
                Utc.with_ymd_and_hms(2023, 1, 1, 14, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2023, 1, 1, 15, 0, 0).unwrap(),
            ),
        ];

        let result = MediaGeoInferrer::find_matching_ride(captured_at, rides);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Test Ride 2");
    }

    #[test]
    fn test_find_matching_ride_matches_preceding_ride() {
        let captured_at = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();

        let rides = vec![
            create_test_ride(
                1,
                Utc.with_ymd_and_hms(2023, 1, 1, 8, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2023, 1, 1, 9, 0, 0).unwrap(),
            ),
            create_test_ride(
                2,
                Utc.with_ymd_and_hms(2023, 1, 1, 10, 15, 0).unwrap(),
                Utc.with_ymd_and_hms(2023, 1, 1, 10, 30, 0).unwrap(),
            ),
        ];

        let result = MediaGeoInferrer::find_matching_ride(captured_at, rides);
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().name,
            "Test Ride 1",
            "Should match the ride that finished before the photo, not the subsequent ride"
        );
    }
}
