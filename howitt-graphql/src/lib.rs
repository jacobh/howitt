#![feature(async_closure)]

pub mod context;
pub mod credentials;
pub mod roles;

use async_graphql::*;
use chrono::{DateTime, Utc};
use context::SchemaData;
use derive_more::From;
use howitt::models::config::ConfigId;
use howitt::models::photo::PhotoId;
use howitt::models::point::ElevationPoint;
use howitt::models::ride::RideId;
use howitt::models::route::RouteId;
use howitt::models::segment_summary::ElevationSummary;
use howitt::models::Model;
use howitt::models::{point_of_interest::PointOfInterestId, ModelRef};
use howitt::services::generate_cuesheet::generate_cuesheet;
use itertools::Itertools;
use roles::Role;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, From)]
pub struct ModelId<ID: howitt::models::ModelId>(ID);

scalar!(ModelId<PointOfInterestId>, "PointOfInterestId");
scalar!(ModelId<RideId>, "RideId");
scalar!(ModelId<RouteId>, "RouteId");
scalar!(ModelId<PhotoId>, "PhotoId");

pub struct Query;

#[Object]
impl Query {
    async fn viewer(&self) -> Viewer {
        Viewer
    }

    async fn routes<'ctx>(&self) -> Result<Vec<Route>, async_graphql::Error> {
        // let route_repo: &RouteModelRepo = ctx.data()?;
        // let routes = route_repo.all_indexes().await?;
        Ok(vec![])
        // Ok(routes.into_iter().map(|route| Route(route)).collect())
    }
    async fn starred_routes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Route>, async_graphql::Error> {
        let SchemaData {
            config_repo,
            route_repo,
            ..
        } = ctx.data()?;

        let config = config_repo.get(ConfigId).await?;

        let routes = route_repo.get_index_batch(config.starred_route_ids).await?;

        Ok(routes
            .into_iter()
            .map(ModelRef::from_index)
            .map(Route)
            .collect())
    }
    async fn published_routes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Route>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;

        let routes = route_repo.all_indexes().await?;

        Ok(routes
            .into_iter()
            .filter(|route| route.published_at().is_some())
            .map(ModelRef::from_index)
            .map(Route)
            .collect())
    }
    async fn route<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: ModelId<RouteId>,
    ) -> Result<Option<Route>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;

        let route = route_repo.get(id.0).await?;

        Ok(Some(Route(ModelRef::from_model(route))))
    }
    async fn rides<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData { ride_repo, .. } = ctx.data()?;
        let rides = ride_repo.all_indexes().await?;

        Ok(rides
            .into_iter()
            .sorted_by_key(|ride| ride.started_at)
            .map(Ride)
            .collect())
    }
    async fn points_of_interest<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<PointOfInterest>, async_graphql::Error> {
        let SchemaData { poi_repo, .. } = ctx.data()?;

        let pois = poi_repo.all_indexes().await?;

        Ok(pois.into_iter().map(PointOfInterest).collect())
    }
    async fn point_of_interest(&self, _ctx: &Context<'_>, _id: usize) -> Option<PointOfInterest> {
        None
    }
}

pub struct Viewer;

#[Object]
impl Viewer {
    async fn role<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Role, async_graphql::Error> {
        Role::from_context(ctx).await
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::route_description::DifficultyRating")]
pub enum DifficultyRating {
    Green,
    Blue,
    Black,
    DoubleBlack,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::route_description::Scouted")]
pub enum Scouted {
    Yes,
    Partially,
    No,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::route_description::Direction")]
pub enum Direction {
    Either,
    PrimarlityAsRouted,
    OnlyAsRouted,
}

pub struct BikeSpec(howitt::models::route_description::BikeSpec);

#[Object]
impl BikeSpec {
    async fn tyre_width(&self) -> Vec<f64> {
        self.0
            .tyre_width
            .clone()
            .map(|x| x.millimeters())
            .into_vec()
    }
    async fn front_suspension(&self) -> Vec<f64> {
        self.0
            .front_suspension
            .clone()
            .map(|x| x.millimeters())
            .into_vec()
    }
    async fn rear_suspension(&self) -> Vec<f64> {
        self.0
            .rear_suspension
            .clone()
            .map(|x| x.millimeters())
            .into_vec()
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::cardinal_direction::CardinalDirection")]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::slope_end::SlopeEnd")]
pub enum SlopeEnd {
    Uphill,
    Downhill,
    Flat,
}

pub struct Terminus {
    terminus: howitt::models::terminus::Terminus<ElevationPoint>,
    route: ModelRef<howitt::models::route::RouteModel>,
}

#[Object]
impl Terminus {
    async fn route(&self) -> Route {
        Route(self.route.clone())
    }
    async fn point(&self) -> Vec<f64> {
        let Terminus { terminus, .. } = self;

        let (x, y) = terminus.point.point.x_y();
        vec![x, y]
    }

    async fn direction(&self) -> CardinalDirection {
        let Terminus { terminus, .. } = self;

        CardinalDirection::from(terminus.direction)
    }

    async fn distance_from_start(&self) -> f64 {
        let Terminus { terminus, .. } = self;

        terminus.distance_from_start
    }

    async fn elevation_gain_from_start(&self) -> Option<f64> {
        let Terminus { terminus, .. } = self;

        terminus
            .elevation
            .as_ref()
            .map(|elevation| elevation.elevation_gain_from_start)
    }

    async fn slope_end(&self) -> Option<SlopeEnd> {
        let Terminus { terminus, .. } = self;

        terminus
            .elevation
            .as_ref()
            .map(|elevation| SlopeEnd::from(elevation.slope_end))
    }
}

pub struct ExternalRef(howitt::models::external_ref::ExternalRef);

#[Object]
impl ExternalRef {
    async fn canonical_url(&self) -> url::Url {
        self.0.id.canonical_url()
    }
}

pub struct Route(ModelRef<howitt::models::route::RouteModel>);

impl Route {
    fn route_description(&self) -> Option<&howitt::models::route_description::RouteDescription> {
        self.0.as_index().description.as_ref()
    }
}

#[Object]
impl Route {
    async fn id(&self) -> ModelId<RouteId> {
        ModelId(self.0.id())
    }
    async fn external_ref(&self) -> Option<ExternalRef> {
        self.0.as_index().external_ref.clone().map(ExternalRef)
    }
    async fn name(&self) -> &str {
        &self.0.as_index().name
    }
    async fn distance(&self) -> f64 {
        self.0.as_index().distance
    }
    async fn elevation_ascent_m<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Option<f64>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.as_model(route_repo).await?;

        Ok(route_model
            .segment_summary()
            .elevation
            .as_ref()
            .map(|summary| summary.elevation_ascent_m))
    }
    async fn elevation_descent_m<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Option<f64>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.as_model(route_repo).await?;

        Ok(route_model
            .segment_summary()
            .elevation
            .as_ref()
            .map(|summary| summary.elevation_descent_m))
    }
    async fn termini(&self) -> Vec<Terminus> {
        self.0
            .as_index()
            .termini
            .as_ref()
            .map(|t| t.to_termini_vec())
            .unwrap_or_default()
            .into_iter()
            .map(|terminus| Terminus {
                terminus,
                route: self.0.clone(),
            })
            .collect_vec()
    }
    async fn description(&self) -> Option<&str> {
        self.route_description()?.description.as_deref()
    }
    async fn photos<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Photo<RouteId>>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.as_model(route_repo).await?;

        Ok(route_model.photos.clone().into_iter().map(Photo).collect())
    }
    async fn technical_difficulty(&self) -> Option<DifficultyRating> {
        self.route_description()?
            .technical_difficulty
            .to_owned()
            .map(DifficultyRating::from)
    }
    async fn physical_difficulty(&self) -> Option<DifficultyRating> {
        self.route_description()?
            .physical_difficulty
            .to_owned()
            .map(DifficultyRating::from)
    }
    async fn minimum_bike(&self) -> Option<BikeSpec> {
        self.route_description()?.minimum_bike.clone().map(BikeSpec)
    }
    async fn ideal_bike(&self) -> Option<BikeSpec> {
        self.route_description()?.ideal_bike.clone().map(BikeSpec)
    }
    async fn scouted(&self) -> Option<Scouted> {
        self.route_description()?
            .scouted
            .to_owned()
            .map(Scouted::from)
    }
    async fn direction(&self) -> Option<Direction> {
        self.route_description()?
            .direction
            .to_owned()
            .map(Direction::from)
    }
    async fn points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.as_model(route_repo).await?;

        Ok(route_model
            .iter_geo_points()
            .map(|point| vec![point.x(), point.y()])
            .collect())
    }
    async fn elevation_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.as_model(route_repo).await?;

        Ok(route_model
            .iter_elevation_points()
            .map(|point| point.elevation)
            .collect())
    }
    async fn distance_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.as_model(route_repo).await?;

        Ok(route_model
            .point_deltas()
            .iter()
            .map(|delta| delta.distance)
            .scan(0.0, |total_distance, distance| {
                *total_distance += distance;

                Some(*total_distance)
            })
            .collect())
    }
    async fn cues<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Cue>, async_graphql::Error> {
        let SchemaData {
            route_repo,
            poi_repo,
            ..
        } = ctx.data()?;
        let route_model = route_repo.get(self.0.id()).await?;

        let points = route_model.iter_elevation_points().cloned().collect_vec();
        let pois = poi_repo.all_indexes().await?;

        let cuesheet = generate_cuesheet(&points, &pois);

        Ok(cuesheet.cues.into_iter().map(Cue::from).collect_vec())
    }
}

pub struct Ride(howitt::models::ride::Ride);

#[Object]
impl Ride {
    async fn id(&self) -> ModelId<RideId> {
        ModelId::from(self.0.id)
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn distance(&self) -> f64 {
        self.0.distance
    }
    async fn started_at(&self) -> DateTime<Utc> {
        self.0.started_at
    }
    async fn finished_at(&self) -> DateTime<Utc> {
        self.0.finished_at
    }
    async fn points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        let SchemaData { ride_repo, .. } = ctx.data()?;
        let ride_model = ride_repo.get(self.0.id).await?;

        Ok(ride_model
            .iter_geo_points()
            .map(|point| vec![point.x(), point.y()])
            .collect())
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::point_of_interest::PointOfInterestType")]
pub enum PointOfInterestType {
    RailwayStation,
    Hut,
    Locality,
    Generic,
}

pub struct PointOfInterest(howitt::models::point_of_interest::PointOfInterest);

#[Object]
impl PointOfInterest {
    async fn id<'a>(&'a self) -> ModelId<PointOfInterestId> {
        ModelId::from(self.0.id())
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn point(&self) -> Vec<f64> {
        vec![self.0.point.x(), self.0.point.y()]
    }
    async fn point_of_interest_type(&self) -> PointOfInterestType {
        PointOfInterestType::from(self.0.point_of_interest_type.clone())
    }
}

pub struct Segment;

#[Object]
impl Segment {
    async fn id(&self) -> usize {
        1
    }
    async fn points(&self) -> Vec<Point> {
        vec![]
    }
}

#[derive(SimpleObject)]
struct Point {
    lat: f64,
    lng: f64,
}

impl From<geo::Point<f64>> for Point {
    fn from(value: geo::Point<f64>) -> Self {
        Point {
            lat: value.y(),
            lng: value.x(),
        }
    }
}

#[derive(SimpleObject)]
pub struct Cue {
    origin: String,
    destination: String,
    distance_meters: f64,
    elevation_ascent_meters: Option<f64>,
    elevation_descent_meters: Option<f64>,
}
impl From<howitt::models::cuesheet::Cue> for Cue {
    fn from(value: howitt::models::cuesheet::Cue) -> Self {
        let (elevation_ascent_meters, elevation_descent_meters) = match value.summary.elevation {
            Some(ElevationSummary {
                elevation_ascent_m,
                elevation_descent_m,
            }) => (Some(elevation_ascent_m), Some(elevation_descent_m)),
            None => (None, None),
        };

        Cue {
            origin: value.origin.to_string(),
            destination: value.destination.to_string(),
            distance_meters: value.summary.distance_m,
            elevation_ascent_meters,
            elevation_descent_meters,
        }
    }
}

pub struct Photo<ID>(howitt::models::photo::Photo<ID>);

#[Object]
impl<ID: howitt::models::ModelId> Photo<ID> {
    async fn id(&self) -> ModelId<PhotoId> {
        ModelId(self.0.id)
    }
    async fn url(&self) -> &url::Url {
        &self.0.url
    }
    async fn caption(&self) -> Option<&str> {
        self.0.caption.as_deref()
    }
}
