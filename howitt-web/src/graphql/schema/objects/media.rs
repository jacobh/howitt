use async_graphql::{Context, Enum, Object, SimpleObject};
use chrono::{DateTime, Utc};
use howitt::{
    models::media::{ImageContentType, ImageSpec, MediaId, IMAGE_SPECS},
    services::media::{generate_resized_media_key, GenerateResizedMediaKeyParams},
};

use crate::graphql::{context::SchemaData, schema::ModelId};

use super::user::UserProfile;

pub struct Media(pub howitt::models::media::Media);

#[Object]
impl Media {
    async fn id(&self) -> ModelId<MediaId> {
        ModelId::from(self.0.id)
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.0.created_at
    }

    async fn path(&self) -> &str {
        &self.0.path
    }

    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserProfile, async_graphql::Error> {
        let SchemaData { user_loader, .. } = ctx.data()?;

        let user = user_loader
            .load_one(self.0.user_id)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?;

        Ok(UserProfile(user))
    }

    async fn image_sizes(&self) -> Vec<ImageSize> {
        const BASE_URL: &str = "https://howitt-media.s3.ap-southeast-4.amazonaws.com/";

        IMAGE_SPECS
            .iter()
            .map(|spec| {
                let (width, height) = spec.dimensions().dimensions();
                let mode = ImageMode::from(spec);

                let jpeg_key = generate_resized_media_key(GenerateResizedMediaKeyParams {
                    media_id: self.0.id,
                    user_id: self.0.user_id,
                    content_type: ImageContentType::Jpeg,
                    image_spec: spec.clone(),
                });

                let webp_key = generate_resized_media_key(GenerateResizedMediaKeyParams {
                    media_id: self.0.id,
                    user_id: self.0.user_id,
                    content_type: ImageContentType::Webp,
                    image_spec: spec.clone(),
                });

                ImageSize {
                    width,
                    height,
                    mode,
                    jpeg_url: format!("{}{}", BASE_URL, jpeg_key),
                    webp_url: format!("{}{}", BASE_URL, webp_key),
                }
            })
            .collect()
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ImageMode {
    Fit,
    Fill,
}

impl From<&ImageSpec> for ImageMode {
    fn from(spec: &ImageSpec) -> Self {
        match spec {
            ImageSpec::Fit(_) => ImageMode::Fit,
            ImageSpec::Fill(_) => ImageMode::Fill,
        }
    }
}

#[derive(SimpleObject)]
pub struct ImageSize {
    width: usize,
    height: usize,
    mode: ImageMode,
    jpeg_url: String,
    webp_url: String,
}
