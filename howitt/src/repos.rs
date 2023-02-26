use async_trait::async_trait;
use std::sync::Arc;

use crate::models::{
    checkpoint::Checkpoint, config::Config, ride::RideModel, route::RouteModel, Model,
};

#[async_trait]
pub trait Repo<T: Model + Sized>: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn all_indexes(&self) -> Result<Vec<T::IndexItem>, Self::Error>;
    async fn get(&self, id: T::Id) -> Result<Option<T>, Self::Error>;
    async fn get_index(&self, id: T::Id) -> Result<Option<T::IndexItem>, Self::Error>;
    async fn get_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T>, Self::Error>;
    async fn put(&self, model: T) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait AnyhowRepo<T: Model + Sized>: Send + Sync {
    async fn all_indexes(&self) -> Result<Vec<T::IndexItem>, anyhow::Error>;
    async fn get(&self, id: T::Id) -> Result<Option<T>, anyhow::Error>;
    async fn get_index(&self, id: T::Id) -> Result<Option<T::IndexItem>, anyhow::Error>;
    async fn get_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T>, anyhow::Error>;
    async fn put(&self, model: T) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl<R, T, E> AnyhowRepo<T> for R
where
    R: Repo<T, Error = E>,
    T: Model + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    async fn all_indexes(&self) -> Result<Vec<T::IndexItem>, anyhow::Error> {
        Ok(Repo::all_indexes(self).await?)
    }
    async fn get(&self, id: T::Id) -> Result<Option<T>, anyhow::Error> {
        Ok(Repo::get(self, id).await?)
    }
    async fn get_index(&self, id: T::Id) -> Result<Option<T::IndexItem>, anyhow::Error> {
        Ok(Repo::get_index(self, id).await?)
    }
    async fn get_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T>, anyhow::Error> {
        Ok(Repo::get_batch(self, ids).await?)
    }
    async fn put(&self, model: T) -> Result<(), anyhow::Error> {
        Ok(Repo::put(self, model).await?)
    }
}

pub type ConfigRepo = Arc<dyn AnyhowRepo<Config>>;
pub type CheckpointRepo = Arc<dyn AnyhowRepo<Checkpoint>>;
pub type RouteModelRepo = Arc<dyn AnyhowRepo<RouteModel>>;
pub type RideModelRepo = Arc<dyn AnyhowRepo<RideModel>>;
