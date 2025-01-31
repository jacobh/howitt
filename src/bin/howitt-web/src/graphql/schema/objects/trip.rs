use std::iter;

use async_graphql::{Context, Object};
use chrono::DateTime;
use howitt::ext::futures::FuturesIteratorExt;
use howitt::ext::iter::{ResultIterExt, ScanAllExt};
use howitt::models::media::MediaFilter;
use howitt::models::{ride::RideFilter, trip::TripId};
use itertools::Itertools;

use crate::graphql::context::SchemaData;
use crate::graphql::schema::TemporalContentBlock;
use crate::graphql::schema::{ride::Ride, ModelId};

use super::media::Media;
use super::note::Note;
use super::user::UserProfile;

pub struct Trip(pub howitt::models::trip::Trip);
pub struct TripLeg(pub Vec<howitt::models::ride::Ride>);

#[Object]
impl TripLeg {
    async fn rides(&self) -> Vec<Ride> {
        self.0.clone().into_iter().map(Ride).collect()
    }
    pub async fn elevation_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let elevations = self
            .0
            .iter()
            .map(|ride| async move { Ride(ride.clone()).elevation_points(ctx).await })
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect_result_vec()?;

        Ok(elevations.into_iter().flatten().collect_vec())
    }

    pub async fn distance_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        // First get all ride distances in parallel
        let distances = self
            .0
            .iter()
            .map(|ride| async move { Ride(ride.clone()).distance_points(ctx).await })
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect_result_vec()?;

        // Then combine them with cumulative offsets
        Ok(distances
            .into_iter()
            .scan_all(0.0, |cumulative_distance, ride_distances| {
                let adjusted_distances = ride_distances
                    .into_iter()
                    .map(|d| d + *cumulative_distance)
                    .collect_vec();

                // Update cumulative distance for next ride
                if let Some(&last) = adjusted_distances.last() {
                    *cumulative_distance = last;
                }

                adjusted_distances
            })
            .flatten()
            .collect())
    }
}

#[Object]
impl Trip {
    async fn id(&self) -> ModelId<TripId> {
        ModelId::from(self.0.id)
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn year(&self) -> i32 {
        self.0.year
    }

    async fn slug(&self) -> &str {
        &self.0.slug
    }

    async fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }

    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserProfile, async_graphql::Error> {
        let SchemaData { user_loader, .. } = ctx.data()?;

        let user = user_loader
            .load_one(self.0.user_id)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?;

        Ok(UserProfile(user))
    }

    async fn legs<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<TripLeg>, async_graphql::Error> {
        let SchemaData { ride_repo, .. } = ctx.data()?;

        let rides = ride_repo
            .filter_models(RideFilter::ForTrip(self.0.id))
            .await?;

        // For this first cut, put all rides in a single leg
        Ok(vec![TripLeg(rides)])
    }

    pub async fn media<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Media>, async_graphql::Error> {
        let SchemaData { media_repo, .. } = ctx.data()?;

        let media = media_repo
            .filter_models(MediaFilter::ForTrip(self.0.id))
            .await?;

        Ok(media.into_iter().map(Media).collect())
    }

    pub async fn temporal_content_blocks<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<TemporalContentBlock>, async_graphql::Error> {
        let SchemaData {
            media_repo,
            ride_repo,
            ..
        } = ctx.data()?;

        // Convert trip description to a Note if it exists
        let notes = self
            .0
            .description
            .as_ref()
            .map(|desc| {
                TemporalContentBlock::Note(Note {
                    content_at: DateTime::from_timestamp(0, 0).unwrap(), // 1970-01-01
                    text: desc.clone(),
                })
            })
            .into_iter();

        // Get media blocks
        let media_blocks = media_repo
            .filter_models(MediaFilter::ForTrip(self.0.id))
            .await?
            .into_iter()
            .map(|m| TemporalContentBlock::Media(Media(m)));

        // Get ride blocks
        let ride_blocks = ride_repo
            .filter_models(RideFilter::ForTrip(self.0.id))
            .await?
            .into_iter()
            .map(|r| TemporalContentBlock::Ride(Ride(r)));

        // Combine all blocks and sort by content_at timestamp
        let blocks: Vec<TemporalContentBlock> = iter::empty()
            .chain(notes)
            .chain(media_blocks)
            .chain(ride_blocks)
            .sorted_by_key(|block| match block {
                TemporalContentBlock::Ride(r) => r.0.started_at,
                TemporalContentBlock::Media(m) => m.0.captured_at.unwrap_or(m.0.created_at),
                TemporalContentBlock::Note(n) => n.content_at,
            })
            .collect_vec();

        Ok(blocks)
    }
}
