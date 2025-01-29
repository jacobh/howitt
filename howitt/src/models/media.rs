use chrono::{DateTime, Utc};
use derive_more::derive::{Display, From};
use serde::{Deserialize, Serialize};

use super::{
    point_of_interest::PointOfInterestId, ride::RideId, route::RouteId, trip::TripId, user::UserId,
    IndexModel, ModelName, ModelUuid,
};

pub type MediaId = ModelUuid<{ ModelName::Media }>;

#[derive(Debug, Clone, From, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaRelationId {
    Ride(RideId),
    Route(RouteId),
    Trip(TripId),
    PointOfInterest(PointOfInterestId),
}
impl MediaRelationId {
    pub fn as_ride_id(&self) -> Option<RideId> {
        match self {
            Self::Ride(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_route_id(&self) -> Option<RouteId> {
        match self {
            Self::Route(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_trip_id(&self) -> Option<TripId> {
        match self {
            Self::Trip(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_point_of_interest_id(&self) -> Option<PointOfInterestId> {
        match self {
            Self::PointOfInterest(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Media {
    pub id: MediaId,
    pub created_at: DateTime<Utc>,
    pub user_id: UserId,
    pub path: String,
    pub relation_ids: Vec<MediaRelationId>,
}

impl Media {
    pub fn iter_ride_ids(&self) -> impl Iterator<Item = RideId> + '_ {
        self.relation_ids
            .iter()
            .filter_map(|relation| relation.as_ride_id())
    }

    pub fn iter_route_ids(&self) -> impl Iterator<Item = RouteId> + '_ {
        self.relation_ids
            .iter()
            .filter_map(|relation| relation.as_route_id())
    }

    pub fn iter_trip_ids(&self) -> impl Iterator<Item = TripId> + '_ {
        self.relation_ids
            .iter()
            .filter_map(|relation| relation.as_trip_id())
    }

    pub fn iter_point_of_interest_ids(&self) -> impl Iterator<Item = PointOfInterestId> + '_ {
        self.relation_ids
            .iter()
            .filter_map(|relation| relation.as_point_of_interest_id())
    }
}

#[derive(Debug, Clone)]
pub enum MediaFilter {
    All,
    ForUser(UserId),
    ForRide(RideId),
    ForRoute(RouteId),
    ForTrip(TripId),
    ForPointOfInterest(PointOfInterestId),
}

impl IndexModel for Media {
    type Id = MediaId;
    type Filter = MediaFilter;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[derive(Debug, Clone, Display)]
pub enum ImageContentType {
    #[display("image/jpeg")]
    Jpeg,
    #[display("image/webp")]
    Webp,
}

impl ImageContentType {
    pub fn as_extension(&self) -> &'static str {
        match self {
            ImageContentType::Jpeg => "jpg",
            ImageContentType::Webp => "webp",
        }
    }
}

#[derive(Debug, Display, Clone)]
pub enum ImageDimensions {
    #[display("{}x{}", _0, _0)]
    Square(usize),
    #[display("{}x{}", width, height)]
    Rectangle { width: usize, height: usize },
}

impl ImageDimensions {
    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            ImageDimensions::Square(size) => (*size, *size),
            ImageDimensions::Rectangle { width, height } => (*width, *height),
        }
    }
}

#[derive(Debug, Display, Clone)]
pub enum ImageSpec {
    #[display("fit_{}", _0)]
    Fit(ImageDimensions),
    #[display("fill_{}", _0)]
    Fill(ImageDimensions),
}

impl ImageSpec {
    pub fn dimensions(&self) -> &ImageDimensions {
        match self {
            ImageSpec::Fit(dimensions) => dimensions,
            ImageSpec::Fill(dimensions) => dimensions,
        }
    }
}

pub const IMAGE_SPECS: &[ImageSpec] = &[
    ImageSpec::Fill(ImageDimensions::Square(300)),
    ImageSpec::Fill(ImageDimensions::Square(600)),
    ImageSpec::Fit(ImageDimensions::Square(800)),
    ImageSpec::Fit(ImageDimensions::Square(1200)),
    ImageSpec::Fit(ImageDimensions::Square(1600)),
    ImageSpec::Fit(ImageDimensions::Square(2000)),
    ImageSpec::Fit(ImageDimensions::Square(2400)),
];

#[cfg(test)]
mod tests {
    use super::{ImageDimensions, ImageSpec};

    #[test]
    fn test_image_dimensions_display() {
        // Test square dimensions
        let square = ImageDimensions::Square(100);
        assert_eq!(square.to_string(), "100x100");

        let large_square = ImageDimensions::Square(2048);
        assert_eq!(large_square.to_string(), "2048x2048");

        // Test rectangle dimensions
        let rectangle = ImageDimensions::Rectangle {
            width: 800,
            height: 600,
        };
        assert_eq!(rectangle.to_string(), "800x600");

        let portrait = ImageDimensions::Rectangle {
            width: 600,
            height: 800,
        };
        assert_eq!(portrait.to_string(), "600x800");
    }

    #[test]
    fn test_image_spec_display() {
        // Test fit dimensions
        let fit_square = ImageSpec::Fit(ImageDimensions::Square(100));
        assert_eq!(fit_square.to_string(), "fit_100x100");

        let fit_rectangle = ImageSpec::Fit(ImageDimensions::Rectangle {
            width: 800,
            height: 600,
        });
        assert_eq!(fit_rectangle.to_string(), "fit_800x600");

        // Test fill dimensions
        let fill_square = ImageSpec::Fill(ImageDimensions::Square(300));
        assert_eq!(fill_square.to_string(), "fill_300x300");

        let fill_rectangle = ImageSpec::Fill(ImageDimensions::Rectangle {
            width: 1024,
            height: 768,
        });
        assert_eq!(fill_rectangle.to_string(), "fill_1024x768");
    }
}
