use crate::ext::futures::FuturesIteratorExt;
use async_trait::async_trait;
use std::sync::Arc;

use crate::ext::iter::ResultIterExt;
use crate::models::{
    config::Config, point_of_interest::PointOfInterest, ride::RideModel, route::RouteModel, Model,
};

#[async_trait]
pub trait Repo<T: Model>: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn all_indexes(&self) -> Result<Vec<T::IndexItem>, Self::Error>;
    async fn get(&self, id: T::Id) -> Result<T, Self::Error>;
    async fn get_index(&self, id: T::Id) -> Result<T::IndexItem, Self::Error>;
    async fn put(&self, model: T) -> Result<(), Self::Error>;

    async fn get_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T>, Self::Error> {
        Ok(ids
            .into_iter()
            .map(|id| (id, self))
            .map(async move |(id, self)| self.get(id).await)
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect_result_vec()?)
    }

    async fn get_index_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T::IndexItem>, Self::Error> {
        Ok(ids
            .into_iter()
            .map(|id| (id, self))
            .map(async move |(id, self)| self.get_index(id).await)
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect_result_vec()?)
    }

    async fn put_batch(&self, models: Vec<T>) -> Result<(), Self::Error> {
        models
            .into_iter()
            .map(|model| (model, self))
            .map(async move |(model, self)| self.put(model).await)
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect_result_vec()?;

        Ok(())
    }
}

#[async_trait]
pub trait AnyhowRepo<T: Model + Sized>: Send + Sync {
    async fn all_indexes(&self) -> Result<Vec<T::IndexItem>, anyhow::Error>;
    async fn get(&self, id: T::Id) -> Result<T, anyhow::Error>;
    async fn get_index(&self, id: T::Id) -> Result<T::IndexItem, anyhow::Error>;
    async fn get_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T>, anyhow::Error>;
    async fn get_index_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T::IndexItem>, anyhow::Error>;
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
    async fn get(&self, id: T::Id) -> Result<T, anyhow::Error> {
        Ok(Repo::get(self, id).await?)
    }
    async fn get_index(&self, id: T::Id) -> Result<T::IndexItem, anyhow::Error> {
        Ok(Repo::get_index(self, id).await?)
    }
    async fn get_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T>, anyhow::Error> {
        Ok(Repo::get_batch(self, ids).await?)
    }
    async fn get_index_batch(&self, ids: Vec<T::Id>) -> Result<Vec<T::IndexItem>, anyhow::Error> {
        Ok(Repo::get_index_batch(self, ids).await?)
    }
    async fn put(&self, model: T) -> Result<(), anyhow::Error> {
        Ok(Repo::put(self, model).await?)
    }
}

pub type ConfigRepo = Arc<dyn AnyhowRepo<Config>>;
pub type PointOfInterestRepo = Arc<dyn AnyhowRepo<PointOfInterest>>;
pub type RouteModelRepo = Arc<dyn AnyhowRepo<RouteModel>>;
pub type RideModelRepo = Arc<dyn AnyhowRepo<RideModel>>;
