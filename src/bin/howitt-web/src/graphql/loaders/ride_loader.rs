use async_graphql::dataloader::Loader;
use howitt::models::ride::{Ride, RideFilter, RideId};
use howitt::repos::RideRepo;
use std::{collections::HashMap, sync::Arc};

pub struct RideLoader {
    ride_repo: RideRepo,
}

impl RideLoader {
    pub fn new(ride_repo: RideRepo) -> Self {
        Self { ride_repo }
    }
}

impl Loader<RideId> for RideLoader {
    type Value = Ride;
    type Error = Arc<anyhow::Error>;

    async fn load(&self, keys: &[RideId]) -> Result<HashMap<RideId, Self::Value>, Self::Error> {
        let rides = self
            .ride_repo
            .filter_models(RideFilter::Ids(keys.to_vec()))
            .await
            .map_err(|e| Arc::new(e))?;

        Ok(rides.into_iter().map(|ride| (ride.id, ride)).collect())
    }
}
