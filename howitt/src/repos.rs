use crate::ext::futures::FuturesIteratorExt;
use crate::models::IndexItem;
use async_trait::async_trait;
use itertools::Itertools;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::OnceCell;

use crate::ext::iter::ResultIterExt;
use crate::models::{
    config::Config, point_of_interest::PointOfInterest, ride::RideModel, route::RouteModel, Model,
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
}

#[async_trait]
pub trait AnyhowRepo: Send + Sync {
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
    async fn put(&self, model: <Self as AnyhowRepo>::Model) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl<R, T, E> AnyhowRepo for R
where
    R: Repo<Model = T, Error = E>,
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
    async fn put(&self, model: T) -> Result<(), anyhow::Error> {
        Ok(Repo::put(self, model).await?)
    }
}

pub struct CachingRepo<R: Repo<Model = M, Error = E>, M: Model, E> {
    repo: R,
    index_cache: OnceCell<Result<std::collections::HashMap<M::Id, M::IndexItem>, Arc<E>>>,
    model_cache: scc::HashMap<M::Id, OnceCell<Result<M, Arc<E>>>>,
}

impl<R, M, E> CachingRepo<R, M, E>
where
    R: Repo<Model = M, Error = E>,
    M: Model,
{
    pub fn new(repo: R) -> CachingRepo<R, M, E> {
        CachingRepo {
            repo,
            index_cache: OnceCell::new(),
            model_cache: scc::HashMap::new(),
        }
    }

    async fn indexes(&self) -> Result<&std::collections::HashMap<M::Id, M::IndexItem>, Arc<E>> {
        self.index_cache
            .get_or_init(async || {
                let indexes = self.repo.all_indexes().await?;

                Ok(std::collections::HashMap::from_iter(
                    indexes.into_iter().map(|x| (x.model_id(), x)),
                ))
            })
            .await
            .as_ref()
            .map_err(|e| e.clone())
    }
}

#[derive(Error, Debug)]
#[error("Caching repo error")]
pub enum CachingRepoError<E: std::error::Error> {
    Upstream(#[from] Arc<E>),
    IndexMissing,
}

#[async_trait]
impl<R, M, E> Repo for CachingRepo<R, M, E>
where
    R: Repo<Model = M, Error = E>,
    M: Model,
    E: std::error::Error + Send + Sync + 'static,
{
    type Model = M;
    type Error = CachingRepoError<E>;

    async fn all_indexes(
        &self,
    ) -> Result<Vec<<<Self as Repo>::Model as Model>::IndexItem>, Self::Error> {
        let indexes = self.indexes().await.map_err(|e| e.clone())?;

        Ok(indexes.clone().into_values().collect_vec())
    }
    async fn get(
        &self,
        id: <<Self as Repo>::Model as Model>::Id,
    ) -> Result<<Self as Repo>::Model, Self::Error> {
        let entry = self.model_cache.entry_async(id).await;

        let entry = entry.or_insert_with(OnceCell::new);

        let model = entry
            .get()
            .get_or_init(async || self.repo.get(id).await.map_err(Arc::new))
            .await
            .as_ref()
            .map_err(|e| e.clone())?;

        Ok((*model).clone())
    }
    async fn get_index(
        &self,
        id: <<Self as Repo>::Model as Model>::Id,
    ) -> Result<<<Self as Repo>::Model as Model>::IndexItem, Self::Error> {
        let indexes = self.indexes().await.map_err(|e| e.clone())?;

        match indexes.get(&id) {
            Some(index) => Ok(index.clone()),
            None => Err(CachingRepoError::IndexMissing),
        }
    }
    async fn put(&self, _model: <Self as Repo>::Model) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

pub type ConfigRepo = Arc<dyn AnyhowRepo<Model = Config>>;
pub type PointOfInterestRepo = Arc<dyn AnyhowRepo<Model = PointOfInterest>>;
pub type RouteModelRepo = Arc<dyn AnyhowRepo<Model = RouteModel>>;
pub type RideModelRepo = Arc<dyn AnyhowRepo<Model = RideModel>>;
