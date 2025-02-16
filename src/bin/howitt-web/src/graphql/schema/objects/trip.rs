use std::iter;

use async_graphql::{Context, Object};
use futures::future::try_join3;
use howitt::models::media::MediaFilter;
use howitt::models::{ride::RideFilter, trip::TripId};
use howitt::repos::Repos;
use howitt::services::fetchers::ElevationPointsParams;
use itertools::Itertools;

use crate::graphql::context::SchemaData;
use crate::graphql::schema::TemporalContentBlock;
use crate::graphql::schema::{ride::Ride, ModelId};

use super::media::Media;
use super::note::Note;
use super::user::UserProfile;

pub struct Trip(pub howitt::models::trip::Trip);
pub struct TripLeg(
    pub howitt::models::trip::Trip,
    pub Vec<howitt::models::ride::Ride>,
);

#[Object]
impl TripLeg {
    async fn rides(&self) -> Vec<Ride> {
        self.1.clone().into_iter().map(Ride).collect()
    }

    async fn tz<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<String>, async_graphql::Error> {
        let first_ride = self.1.first();

        if let Some(ride) = first_ride {
            let ride = Ride(ride.clone());

            ride.tz(ctx).await
        } else {
            Ok(None)
        }
    }

    pub async fn elevation_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData {
            simplified_trip_elevation_points_fetcher,
            ..
        } = ctx.data()?;

        let trip_id = self.0.id;

        let points = simplified_trip_elevation_points_fetcher
            .fetch(trip_id, ElevationPointsParams::default())
            .await?;

        Ok(points.into_iter().map(|p| p.1).collect())
    }
    pub async fn distance_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData {
            simplified_trip_elevation_points_fetcher,
            ..
        } = ctx.data()?;

        let trip_id = self.0.id;

        let points = simplified_trip_elevation_points_fetcher
            .fetch(trip_id, ElevationPointsParams::default())
            .await?;

        Ok(points.into_iter().map(|p| p.0).collect())
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

    async fn is_published(&self) -> bool {
        self.0.is_published
    }

    async fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }

    async fn tz<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<String>, async_graphql::Error> {
        let rides = self.rides(ctx).await?;
        let first_ride = rides.first();

        if let Some(ride) = first_ride {
            ride.tz(ctx).await
        } else {
            Ok(None)
        }
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
            self.0.clone(),
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

    async fn notes<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Note>, async_graphql::Error> {
        let rides = {
            let mut rides = self.rides(ctx).await?;
            rides.sort_by_key(|ride| ride.0.started_at);
            rides
        };

        Ok(self
            .0
            .notes
            .iter()
            .map(|note| {
                let matching_ride = rides
                    .iter()
                    .rev()
                    .find(|ride| ride.0.started_at <= note.timestamp)
                    .cloned();

                Note {
                    content_at: note.timestamp,
                    text: note.text.clone(),
                    ride: matching_ride,
                }
            })
            .collect())
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
