use itertools::Itertools;
use thiserror::Error;

use crate::{
    ext::{futures::FuturesIteratorExt, iter::ResultIterExt},
    models::{
        external_ref::{ExternalId, ExternalRef, RwgpsId},
        photo::Photo,
        route::RouteModel,
        ModelId,
    },
    repos::Repo,
};

use howitt_client_types::{BucketClient, HttpClient, HttpResponse};

#[derive(Debug, derive_more::Display, Error)]
#[display("{:#?}", "_0")]
pub enum PhotoSyncError<BC: BucketClient, HC: HttpClient, RouteRepo: Repo<Model = RouteModel>> {
    PhotoExistsFailed(BC::Error),
    PhotoPutFailed(BC::Error),
    PhotoDownloadFailed(HC::Error),
    RouteRepoError(RouteRepo::Error),
}

pub struct PhotoSyncService<BC: BucketClient, HC: HttpClient, RouteRepo: Repo<Model = RouteModel>> {
    pub bucket_client: BC,
    pub http_client: HC,
    pub route_repo: RouteRepo,
}

impl<BC, HC, RouteRepo> PhotoSyncService<BC, HC, RouteRepo>
where
    BC: BucketClient,
    HC: HttpClient,
    RouteRepo: Repo<Model = RouteModel>,
{
    pub type Error = PhotoSyncError<BC, HC, RouteRepo>;

    pub async fn photo_exists<ID: ModelId>(&self, photo: &Photo<ID>) -> Result<bool, Self::Error> {
        self.bucket_client
            .key_exists(&make_key(photo))
            .await
            .map_err(PhotoSyncError::PhotoExistsFailed)
    }

    pub async fn fetch_source_photo<ID: ModelId>(
        &self,
        photo: &Photo<ID>,
    ) -> Result<bytes::Bytes, Self::Error> {
        let HttpResponse { body } = self
            .http_client
            .get(make_source_url(photo))
            .await
            .map_err(PhotoSyncError::PhotoDownloadFailed)?;

        Ok(body)
    }

    pub async fn put_photo<ID: ModelId>(
        &self,
        photo: &Photo<ID>,
        bytes: bytes::Bytes,
    ) -> Result<(), Self::Error> {
        self.bucket_client
            .put_object(&make_key(photo), bytes)
            .await
            .map_err(PhotoSyncError::PhotoPutFailed)?;

        Ok(())
    }

    pub async fn sync_photo<ID: ModelId>(&self, photo: &Photo<ID>) -> Result<(), Self::Error> {
        if !self.photo_exists(photo).await? {
            let bytes = self.fetch_source_photo(photo).await?;
            self.put_photo(photo, bytes).await?;
            println!("synced photo: {photo:#?}");
        }

        Ok(())
    }

    pub async fn sync(&self) -> Result<(), Self::Error> {
        let routes = self
            .route_repo
            .all_models()
            .await
            .map_err(PhotoSyncError::RouteRepoError)?;

        let photos = routes
            .into_iter()
            .flat_map(|route| route.photos)
            .collect_vec();

        photos
            .into_iter()
            .map(|photo| (photo, self))
            .map(async move |(photo, sync)| sync.sync_photo(&photo).await)
            .collect_futures_ordered()
            .await
            .into_iter()
            .collect_result_vec()?;

        Ok(())
    }
}

fn make_key<ID: ModelId>(photo: &Photo<ID>) -> String {
    let id = photo.id.as_uuid();

    format!("source/{id}.jpg")
}

fn make_source_url<ID: ModelId>(photo: &Photo<ID>) -> url::Url {
    let rwgps_photo_id = match photo.external_ref {
        Some(ExternalRef {
            id: ExternalId::Rwgps(RwgpsId::Photo(photo_id)),
            ..
        }) => Some(photo_id),
        _ => None,
    }
    .unwrap();

    url::Url::parse(&format!(
        "https://ridewithgps.com/photos/{rwgps_photo_id}/full.jpg"
    ))
    .unwrap()
}
