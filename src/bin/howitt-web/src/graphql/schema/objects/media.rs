use async_graphql::{Context, Enum, Object, SimpleObject};
use chrono::{DateTime, Utc};
use howitt::{
    models::media::{ImageContentType, ImageSpec, MediaId, IMAGE_SPECS},
    services::media::{generate_resized_media_key, GenerateResizedMediaKeyParams},
};
use itertools::Itertools;

use crate::graphql::{context::SchemaData, schema::ModelId};

use super::{ride::Ride, user::UserProfile};

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

    async fn point(&self) -> Option<Vec<f64>> {
        self.0.point.map(|p| vec![p.x(), p.y()])
    }

    async fn captured_at(&self) -> Option<DateTime<Utc>> {
        self.0.captured_at
    }

    async fn tz<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<String>, async_graphql::Error> {
        let SchemaData { tz_finder, .. } = ctx.data()?;

        Ok(self
            .0
            .point
            .as_ref()
            .map(|point| tz_finder.get_tz_name(point.x(), point.y()).to_string()))
    }

    pub async fn content_at(&self) -> DateTime<Utc> {
        self.0.captured_at.unwrap_or(self.0.created_at).clone()
    }

    async fn image_sizes(&self) -> ImageSizes {
        const BASE_URL: &str = "https://d36p712mevhglz.cloudfront.net/";

        ImageSizes {
            fill_300: self.create_image_size(&IMAGE_SPECS[0], BASE_URL),
            fill_600: self.create_image_size(&IMAGE_SPECS[1], BASE_URL),
            fit_800: self.create_image_size(&IMAGE_SPECS[2], BASE_URL),
            fit_1200: self.create_image_size(&IMAGE_SPECS[3], BASE_URL),
            fit_1600: self.create_image_size(&IMAGE_SPECS[4], BASE_URL),
            fit_2000: self.create_image_size(&IMAGE_SPECS[5], BASE_URL),
            fit_2400: self.create_image_size(&IMAGE_SPECS[6], BASE_URL),
        }
    }

    async fn rides<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData { ride_loader, .. } = ctx.data()?;

        let rides = ride_loader
            .load_many(self.0.iter_ride_ids().collect_vec())
            .await?;

        Ok(rides.into_values().map(Ride).collect_vec())
    }
}

impl Media {
    fn create_image_size(&self, spec: &ImageSpec, base_url: &str) -> ImageSize {
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
            jpeg_url: format!("{}{}", base_url, jpeg_key),
            webp_url: format!("{}{}", base_url, webp_key),
        }
    }
}

#[derive(SimpleObject)]
pub struct ImageSizes {
    fill_300: ImageSize,
    fill_600: ImageSize,
    fit_800: ImageSize,
    fit_1200: ImageSize,
    fit_1600: ImageSize,
    fit_2000: ImageSize,
    fit_2400: ImageSize,
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

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use howitt::models::media::{ImageDimensions, ImageSpec};

    #[test]
    fn test_image_sizes_matches_image_specs() {
        let type_info = std::any::TypeId::of::<ImageSizes>();
        let type_name = std::any::type_name::<ImageSizes>();

        // Get the number of fields in ImageSizes struct using std::mem::size_of
        let image_sizes_field_count =
            std::mem::size_of::<ImageSizes>() / std::mem::size_of::<ImageSize>();

        // Check if the number of fields matches IMAGE_SPECS
        assert_eq!(
            image_sizes_field_count,
            IMAGE_SPECS.len(),
            "ImageSizes struct has {} fields but IMAGE_SPECS has {} entries. They must have the same number of entries.\nType: {}\nTypeId: {:?}",
            image_sizes_field_count,
            IMAGE_SPECS.len(),
            type_name,
            type_info,
        );

        // Verify each spec matches its expected field name pattern
        for (i, spec) in IMAGE_SPECS.iter().enumerate() {
            let expected_field_name = match spec {
                ImageSpec::Fill(ImageDimensions::Square(size)) => format!("fill_{}", size),
                ImageSpec::Fit(ImageDimensions::Square(size)) => format!("fit_{}", size),
                _ => panic!("Unexpected ImageSpec format at index {}", i),
            };

            // This will fail compilation if we add a new IMAGE_SPEC without updating ImageSizes
            match i {
                0 => assert_eq!(expected_field_name, "fill_300"),
                1 => assert_eq!(expected_field_name, "fill_600"),
                2 => assert_eq!(expected_field_name, "fit_800"),
                3 => assert_eq!(expected_field_name, "fit_1200"),
                4 => assert_eq!(expected_field_name, "fit_1600"),
                5 => assert_eq!(expected_field_name, "fit_2000"),
                6 => assert_eq!(expected_field_name, "fit_2400"),
                _ => panic!("Found more IMAGE_SPECS than fields in ImageSizes struct"),
            }
        }
    }
}
