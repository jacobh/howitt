use async_graphql::dataloader::Loader;
use futures::future::try_join_all;
use howitt::{
    models::{
        media::{Media, MediaFilter},
        ride::{Ride, RideFilter},
        trip::TripId,
    },
    repos::{MediaRepo, RideRepo},
};
use std::{collections::HashMap, sync::Arc};

pub struct TripRidesLoader(pub RideRepo);
pub struct TripMediaLoader(pub MediaRepo);

impl Loader<TripId> for TripRidesLoader {
    type Value = Vec<Ride>;
    type Error = Arc<anyhow::Error>;

    async fn load(&self, keys: &[TripId]) -> Result<HashMap<TripId, Self::Value>, Self::Error> {
        try_join_all(keys.iter().map(|&id| async move {
            self.0
                .filter_models(RideFilter::ForTrip(id))
                .await
                .map(|rides| (id, rides))
                .map_err(Arc::new)
        }))
        .await
        .map(|entries| entries.into_iter().collect())
    }
}

impl Loader<TripId> for TripMediaLoader {
    type Value = Vec<Media>;
    type Error = Arc<anyhow::Error>;

    async fn load(&self, keys: &[TripId]) -> Result<HashMap<TripId, Self::Value>, Self::Error> {
        try_join_all(keys.iter().map(|&id| async move {
            self.0
                .filter_models(MediaFilter::ForTrip(id))
                .await
                .map(|media| (id, media))
                .map_err(Arc::new)
        }))
        .await
        .map(|entries| entries.into_iter().collect())
    }
}
