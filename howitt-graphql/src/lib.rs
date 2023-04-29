#![feature(async_closure)]

pub mod context;
pub mod credentials;
pub mod roles;

use async_graphql::*;
use chrono::{DateTime, Utc};
use context::SchemaData;
use derive_more::From;
use geo::CoordsIter;
use howitt::models::config::ConfigId;
use howitt::models::ride::RideId;
use howitt::models::route::RouteId;
use howitt::models::Model;
use howitt::models::{checkpoint::CheckpointId, ModelRef};
use itertools::Itertools;
use roles::Role;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, From)]
pub struct ModelId<ID: howitt::models::ModelId>(ID);

scalar!(ModelId<CheckpointId>, "CheckpointId");
scalar!(ModelId<RideId>, "RideId");
scalar!(ModelId<RouteId>, "RouteId");

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
    async fn checkpoints<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Checkpoint>, async_graphql::Error> {
        let SchemaData {
            checkpoint_repo, ..
        } = ctx.data()?;

        let checkpoints = checkpoint_repo.all_indexes().await?;

        Ok(checkpoints.into_iter().map(Checkpoint).collect())
    }
    async fn checkpoint(&self, _ctx: &Context<'_>, _id: usize) -> Option<Checkpoint> {
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

pub struct Route(ModelRef<howitt::models::route::RouteModel>);

#[Object]
impl Route {
    async fn id(&self) -> ModelId<RouteId> {
        ModelId(self.0.id())
    }
    async fn name(&self) -> &str {
        &self.0.as_index().name
    }
    async fn distance(&self) -> f64 {
        self.0.as_index().distance
    }
    async fn geojson<'ctx>(&self, ctx: &Context<'ctx>) -> Result<String, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = route_repo.get(self.0.id()).await?;

        let linestring = geo::LineString::from(route_model.iter_geo_points().collect::<Vec<_>>());
        Ok(geojson::Feature::from(geojson::Geometry::try_from(&linestring).unwrap()).to_string())
    }
    async fn points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = route_repo.get(self.0.id()).await?;

        Ok(route_model
            .iter_geo_points()
            .map(|point| vec![point.x(), point.y()])
            .collect())
    }
    async fn polyline<'ctx>(&self, ctx: &Context<'ctx>) -> Result<String, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = route_repo.get(self.0.id()).await?;

        Ok(polyline::encode_coordinates(
            route_model
                .iter_geo_points()
                .flat_map(|point| point.coords_iter()),
            5,
        )?)
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
    async fn geojson<'ctx>(&self, ctx: &Context<'ctx>) -> Result<String, async_graphql::Error> {
        let SchemaData { ride_repo, .. } = ctx.data()?;
        let ride_model = ride_repo.get(self.0.id).await?;

        let linestring = geo::LineString::from_iter(ride_model.iter_geo_points());

        Ok(geojson::Feature::from(geojson::Geometry::try_from(&linestring).unwrap()).to_string())
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
#[graphql(remote = "howitt::models::checkpoint::CheckpointType")]
pub enum CheckpointType {
    RailwayStation,
    Hut,
    Locality,
    Generic,
}

pub struct Checkpoint(howitt::models::checkpoint::Checkpoint);

#[Object]
impl Checkpoint {
    async fn id<'a>(&'a self) -> ModelId<CheckpointId> {
        ModelId::from(self.0.id())
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn point(&self) -> Vec<f64> {
        vec![self.0.point.x(), self.0.point.y()]
    }
    async fn checkpoint_type(&self) -> CheckpointType {
        CheckpointType::from(self.0.checkpoint_type.clone())
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
