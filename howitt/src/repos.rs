use crate::ext::futures::FuturesIteratorExt;
use crate::models::IndexItem;
use crate::models::user::User;
use async_trait::async_trait;
use std::sync::Arc;

use crate::ext::iter::ResultIterExt;
use crate::models::{
    Model, config::Config, point_of_interest::PointOfInterest, ride::RideModel, route::RouteModel,
};

#[async_trait]
pub trait Repo: Send + Sync {
    type Model: Model;
    type Error: std::error::Error + Send + Sync + 'static;

    async fn all_indexes(
        &self,
    ) -> Result<Vec<<<Self as Repo>::Model as Model>::IndexItem>, Self::Error>;
    async fn get(
        &self,
        id: <<Self as Repo>::Model as Model>::Id,
    ) -> Result<<Self as Repo>::Model, Self::Error>;
    async fn get_index(
        &self,
        id: <<Self as Repo>::Model as Model>::Id,
    ) -> Result<<<Self as Repo>::Model as Model>::IndexItem, Self::Error>;
    async fn put(&self, model: <Self as Repo>::Model) -> Result<(), Self::Error>;

    async fn get_batch(
        &self,
        ids: Vec<<<Self as Repo>::Model as Model>::Id>,
    ) -> Result<Vec<<Self as Repo>::Model>, Self::Error> {
        Ok(ids
            .into_iter()
            .map(|id| (id, self))
            .map(async move |(id, self)| self.get(id).await)
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect_result_vec()?)
    }

    async fn get_index_batch(
        &self,
        ids: Vec<<<Self as Repo>::Model as Model>::Id>,
    ) -> Result<Vec<<<Self as Repo>::Model as Model>::IndexItem>, Self::Error> {
        Ok(ids
            .into_iter()
            .map(|id| (id, self))
            .map(async move |(id, self)| self.get_index(id).await)
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect_result_vec()?)
    }

    async fn put_batch(&self, models: Vec<<Self as Repo>::Model>) -> Result<(), Self::Error> {
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

    async fn filter_models(
        &self,
        filter: <Self::Model as Model>::Filter,
    ) -> Result<Vec<Self::Model>, Self::Error>;

    async fn find_model(
        &self,
        filter: <Self::Model as Model>::Filter,
    ) -> Result<Option<Self::Model>, Self::Error> {
        let models = self.filter_models(filter).await?;

        Ok(models.into_iter().nth(0))
    }

    async fn all_models(&self) -> Result<Vec<<Self as Repo>::Model>, Self::Error> {
        let indexes = self.all_indexes().await?;

        self.get_batch(indexes.into_iter().map(|index| index.model_id()).collect())
            .await
    }
}

#[async_trait]
pub trait AnyhowRepo: Send + Sync + std::fmt::Debug {
    type Model: Model + Sized;

    async fn all_indexes(
        &self,
    ) -> Result<Vec<<<Self as AnyhowRepo>::Model as Model>::IndexItem>, anyhow::Error>;
    async fn get(
        &self,
        id: <<Self as AnyhowRepo>::Model as Model>::Id,
    ) -> Result<<Self as AnyhowRepo>::Model, anyhow::Error>;
    async fn get_index(
        &self,
        id: <<Self as AnyhowRepo>::Model as Model>::Id,
    ) -> Result<<<Self as AnyhowRepo>::Model as Model>::IndexItem, anyhow::Error>;
    async fn get_batch(
        &self,
        ids: Vec<<<Self as AnyhowRepo>::Model as Model>::Id>,
    ) -> Result<Vec<<Self as AnyhowRepo>::Model>, anyhow::Error>;
    async fn get_index_batch(
        &self,
        ids: Vec<<<Self as AnyhowRepo>::Model as Model>::Id>,
    ) -> Result<Vec<<<Self as AnyhowRepo>::Model as Model>::IndexItem>, anyhow::Error>;
    async fn filter_models(
        &self,
        filter: <Self::Model as Model>::Filter,
    ) -> Result<Vec<Self::Model>, anyhow::Error>;
    async fn find_model(
        &self,
        filter: <Self::Model as Model>::Filter,
    ) -> Result<Option<Self::Model>, anyhow::Error>;
    async fn put(&self, model: <Self as AnyhowRepo>::Model) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl<R, T, E> AnyhowRepo for R
where
    R: Repo<Model = T, Error = E> + std::fmt::Debug,
    T: Model + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    type Model = T;

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
    async fn filter_models(
        &self,
        filter: <Self::Model as Model>::Filter,
    ) -> Result<Vec<Self::Model>, anyhow::Error> {
        Ok(Repo::filter_models(self, filter).await?)
    }
    async fn find_model(
        &self,
        filter: <Self::Model as Model>::Filter,
    ) -> Result<Option<Self::Model>, anyhow::Error> {
        Ok(Repo::find_model(self, filter).await?)
    }
    async fn put(&self, model: T) -> Result<(), anyhow::Error> {
        Ok(Repo::put(self, model).await?)
    }
}

pub type ConfigRepo = Arc<dyn AnyhowRepo<Model = Config>>;
pub type PointOfInterestRepo = Arc<dyn AnyhowRepo<Model = PointOfInterest>>;
pub type RouteModelRepo = Arc<dyn AnyhowRepo<Model = RouteModel>>;
pub type RideModelRepo = Arc<dyn AnyhowRepo<Model = RideModel>>;
pub type UserRepo = Arc<dyn AnyhowRepo<Model = User>>;
