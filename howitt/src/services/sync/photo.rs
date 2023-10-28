use thiserror::Error;

use crate::models::{
    external_ref::{ExternalId, ExternalRef, RwgpsId},
    photo::Photo,
    ModelId,
};

use howitt_client_types::{BucketClient, HttpClient, HttpResponse};

#[derive(Debug, Error)]
pub enum PhotoSyncError<BC: BucketClient, HC: HttpClient> {
    PhotoExistsFailed(BC::Error),
    PhotoPutFailed(BC::Error),
    PhotoDownloadFailed(HC::Error),
}

pub struct PhotoSyncService<BC: BucketClient, HC: HttpClient> {
    pub bucket_client: BC,
    pub http_client: HC,
}

impl<BC, HC> PhotoSyncService<BC, HC>
where
    BC: BucketClient,
    HC: HttpClient,
{
    pub type Error = PhotoSyncError<BC, HC>;

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
        }

        Ok(())
    }
}

fn make_key<ID: ModelId>(photo: &Photo<ID>) -> String {
    let id = photo.id;

    format!("/photos/{id}.jpg")
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
        "https://ridewithgps.com/photos/{rwgps_photo_id}/large.jpg"
    ))
    .unwrap()
}
