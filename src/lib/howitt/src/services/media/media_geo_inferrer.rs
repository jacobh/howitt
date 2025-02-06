use chrono::Duration;
use thiserror::Error;

use crate::{
    models::{filters::TemporalFilter, media::Media, point::Point, ride::RideFilter},
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

    pub async fn infer_point(&self, media: &Media) -> Result<Option<geo::Point>, anyhow::Error> {
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

        // Find the ride that was ongoing when the media was captured
        let matching_ride = candidate_rides
            .into_iter()
            .find(|ride| captured_at >= ride.started_at && captured_at <= ride.finished_at);

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

        Ok(closest_point)
    }

    pub async fn infer_point_and_save(&self, media: &Media) -> Result<(), anyhow::Error> {
        if let Some(point) = self.infer_point(media).await? {
            let mut updated_media = media.clone();
            updated_media.point = Some(point);
            self.media_repo.put(updated_media).await?;
        }

        Ok(())
    }
}
