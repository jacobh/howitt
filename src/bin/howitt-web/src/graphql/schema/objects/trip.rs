use std::iter;

use async_graphql::{Context, Object};
use futures::future::try_join3;
use howitt::ext::futures::FuturesIteratorExt;
use howitt::ext::iter::{ResultIterExt, ScanAllExt};
use howitt::models::media::MediaFilter;
use howitt::models::{ride::RideFilter, trip::TripId};
use howitt::repos::Repos;
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
    pub async fn elevation_points_json<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<String, async_graphql::Error> {
        Ok(serde_json::to_string(&self.elevation_points(ctx).await?)?)
    }
    pub async fn distance_points_json<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<String, async_graphql::Error> {
        Ok(serde_json::to_string(&self.distance_points(ctx).await?)?)
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

    async fn rides<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { ride_repo, .. },
            ..
        } = ctx.data()?;

        let rides = ride_repo
            .filter_models(RideFilter::ForTrip(self.0.id))
            .await?;

        Ok(rides.into_iter().map(Ride).collect())
    }

    async fn legs<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<TripLeg>, async_graphql::Error> {
        let rides = self.rides(ctx).await?;

        // For this first cut, put all rides in a single leg
        Ok(vec![TripLeg(
            rides.into_iter().map(|ride| ride.0).collect(),
        )])
    }

    pub async fn media<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Media>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { media_repo, .. },
            ..
        } = ctx.data()?;

        let media = media_repo
            .filter_models(MediaFilter::ForTrip(self.0.id))
            .await?;

        Ok(media.into_iter().map(Media).collect())
    }

    async fn notes(&self) -> Vec<Note> {
        self.0
            .notes
            .iter()
            .map(|note| Note {
                content_at: note.timestamp,
                text: note.text.clone(),
            })
            .collect()
    }

    pub async fn temporal_content_blocks<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<TemporalContentBlock>, async_graphql::Error> {
        let (notes, media, rides) =
            try_join3(self.notes(ctx), self.media(ctx), self.rides(ctx)).await?;

        let note_blocks = notes.into_iter().map(TemporalContentBlock::Note);

        let media_blocks = media.into_iter().map(TemporalContentBlock::Media);

        let ride_blocks = rides.into_iter().map(TemporalContentBlock::Ride);

        let blocks: Vec<TemporalContentBlock> = iter::empty()
            .chain(note_blocks)
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
