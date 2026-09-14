use sanitize_filename::sanitize;

use crate::models::{
    media::{ImageContentType, ImageSpec, MediaId},
    user::UserId,
};

/// Provider-qualified locator for new uploads. Legacy paths remain S3 keys.
/// The delivery hash is public, unlike the account API credential.
pub fn cloudflare_image_path(hash: &str, id: &str) -> Option<String> {
    let valid = |part: &str| {
        !part.is_empty()
            && part.len() <= 64
            && part
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    };
    (valid(hash) && valid(id)).then(|| format!("cloudflare-images://{hash}/{id}"))
}

pub fn cloudflare_image_url(
    path: &str,
    spec: &ImageSpec,
    format: ImageContentType,
) -> Option<String> {
    let (hash, id) = path.strip_prefix("cloudflare-images://")?.split_once('/')?;
    cloudflare_image_path(hash, id)?;
    let (width, height) = spec.dimensions().dimensions();
    let fit = match spec {
        ImageSpec::Fit(_) => "scale-down",
        ImageSpec::Fill(_) => "cover",
    };
    let format = match format {
        ImageContentType::Jpeg => "jpeg",
        ImageContentType::Webp => "webp",
    };
    Some(format!(
        "https://imagedelivery.net/{hash}/{id}/width={width},height={height},fit={fit},format={format},quality=85,metadata=none"
    ))
}

pub struct GenerateMediaKeyParams {
    pub media_id: MediaId,
    pub user_id: UserId,
    pub name: String,
}

pub fn generate_media_key(
    GenerateMediaKeyParams {
        media_id,
        user_id,
        name,
    }: GenerateMediaKeyParams,
) -> String {
    format!(
        "originals/user/{}/media/{}/{}",
        user_id.as_uuid(),
        media_id.as_uuid(),
        sanitize(&name)
    )
}

pub struct GenerateResizedMediaKeyParams {
    pub media_id: MediaId,
    pub user_id: UserId,
    pub content_type: ImageContentType,
    pub image_spec: ImageSpec,
}

pub fn generate_resized_media_key(
    GenerateResizedMediaKeyParams {
        media_id,
        user_id,
        content_type,
        image_spec,
    }: GenerateResizedMediaKeyParams,
) -> String {
    format!(
        "resizes/user/{}/media/{}_{}.{}",
        user_id.as_uuid(),
        media_id.as_uuid(),
        image_spec,
        content_type.as_extension()
    )
}

#[cfg(test)]
mod tests {
    use crate::models::media::ImageDimensions;

    use super::*;

    #[test]
    fn cloudflare_urls_preserve_dimensions_modes_and_explicit_formats() {
        let path = cloudflare_image_path("public_hash", "image-id").unwrap();
        assert_eq!(
            cloudflare_image_url(
                &path,
                &ImageSpec::Fit(ImageDimensions::Rectangle {
                    width: 1200,
                    height: 800
                }),
                ImageContentType::Jpeg
            )
            .unwrap(),
            "https://imagedelivery.net/public_hash/image-id/width=1200,height=800,fit=scale-down,format=jpeg,quality=85,metadata=none"
        );
        assert_eq!(
            cloudflare_image_url(
                &path,
                &ImageSpec::Fill(ImageDimensions::Square(300)),
                ImageContentType::Webp
            )
            .unwrap(),
            "https://imagedelivery.net/public_hash/image-id/width=300,height=300,fit=cover,format=webp,quality=85,metadata=none"
        );
        for path in [
            "originals/legacy.jpg",
            "cloudflare-images://hash/../other",
            "cloudflare-images://hash/id?metadata=keep",
            "cloudflare-images:///id",
        ] {
            assert!(
                cloudflare_image_url(
                    path,
                    &ImageSpec::Fill(ImageDimensions::Square(300)),
                    ImageContentType::Jpeg
                )
                .is_none()
            );
        }
    }

    #[test]
    fn test_generate_media_key() {
        let params = GenerateMediaKeyParams {
            media_id: MediaId::from(
                uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            ),
            user_id: UserId::from(
                uuid::Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            ),
            name: "test-image.jpg".to_string(),
        };

        let key = generate_media_key(params);
        assert_eq!(
            key,
            "originals/user/123e4567-e89b-12d3-a456-426614174000/media/550e8400-e29b-41d4-a716-446655440000/test-image.jpg"
        );
    }

    #[test]
    fn test_generate_media_key_with_special_chars() {
        let params = GenerateMediaKeyParams {
            media_id: MediaId::from(
                uuid::Uuid::parse_str("7f9c24e5-2c44-4a8e-95d3-a515bf484018").unwrap(),
            ),
            user_id: UserId::from(
                uuid::Uuid::parse_str("9d25a949-c374-4f22-9ca8-1c17d4982384").unwrap(),
            ),
            name: "test image/with spaces!@#$.jpg".to_string(),
        };

        let key = generate_media_key(params);
        assert_eq!(
            key,
            "originals/user/9d25a949-c374-4f22-9ca8-1c17d4982384/media/7f9c24e5-2c44-4a8e-95d3-a515bf484018/test imagewith spaces!@#$.jpg"
        );
    }

    #[test]
    fn test_generate_resized_media_key() {
        let params = GenerateResizedMediaKeyParams {
            media_id: MediaId::from(
                uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            ),
            user_id: UserId::from(
                uuid::Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            ),
            content_type: ImageContentType::Jpeg,
            image_spec: ImageSpec::Fill(ImageDimensions::Square(300)),
        };

        let key = generate_resized_media_key(params);
        assert_eq!(
            key,
            "resizes/user/123e4567-e89b-12d3-a456-426614174000/media/550e8400-e29b-41d4-a716-446655440000_fill_300x300.jpg"
        );
    }

    #[test]
    fn test_generate_resized_media_key_with_different_format() {
        let params = GenerateResizedMediaKeyParams {
            media_id: MediaId::from(
                uuid::Uuid::parse_str("7f9c24e5-2c44-4a8e-95d3-a515bf484018").unwrap(),
            ),
            user_id: UserId::from(
                uuid::Uuid::parse_str("9d25a949-c374-4f22-9ca8-1c17d4982384").unwrap(),
            ),
            content_type: ImageContentType::Webp,
            image_spec: ImageSpec::Fit(ImageDimensions::Rectangle {
                width: 800,
                height: 600,
            }),
        };

        let key = generate_resized_media_key(params);
        assert_eq!(
            key,
            "resizes/user/9d25a949-c374-4f22-9ca8-1c17d4982384/media/7f9c24e5-2c44-4a8e-95d3-a515bf484018_fit_800x600.webp"
        );
    }
}
