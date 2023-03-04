#![feature(async_closure)]
use async_graphql::*;
use derive_more::From;
use geo::CoordsIter;
use howitt::models::checkpoint::CheckpointId;
use howitt::models::config::ConfigId;
use howitt::models::ride::RideId;
use howitt::models::route::{RouteId, RouteModel};
use howitt::models::Model;
use howitt::repos::{CheckpointRepo, ConfigRepo, RideModelRepo, RouteModelRepo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, From)]
pub struct ModelId<ID: howitt::models::ModelId>(ID);

scalar!(ModelId<CheckpointId>, "CheckpointId");
scalar!(ModelId<RideId>, "RideId");
scalar!(ModelId<RouteId>, "RouteId");

pub struct Query;

#[Object]
impl Query {
    async fn routes<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Route>, async_graphql::Error> {
        // let route_repo: &RouteModelRepo = ctx.data()?;
        // let routes = route_repo.all_indexes().await?;
        Ok(vec![])
        // Ok(routes.into_iter().map(|route| Route(route)).collect())
    }
    async fn starred_routes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Route>, async_graphql::Error> {
        let config_repo: &ConfigRepo = ctx.data()?;
        let route_repo: &RouteModelRepo = ctx.data()?;

        let config = config_repo.get(ConfigId).await?.unwrap_or_default();

        let routes = route_repo.get_batch(config.starred_route_ids).await?;

        Ok(routes.into_iter().map(Route).collect())
    }
    async fn route(&self, _ctx: &Context<'_>, _id: usize) -> Option<Route> {
        None
    }
    async fn rides<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Ride>, async_graphql::Error> {
        let ride_repo: &RideModelRepo = ctx.data()?;
        let rides = ride_repo.all_indexes().await?;

        Ok(rides.into_iter().map(|ride| Ride(ride)).collect())
    }
    async fn checkpoints<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Checkpoint>, async_graphql::Error> {
        let checkpoint_repo: &CheckpointRepo = ctx.data()?;

        let checkpoints = checkpoint_repo.all_indexes().await?;

        Ok(checkpoints
            .into_iter()
            .map(|checkpoint| Checkpoint(checkpoint))
            .collect())
    }
    async fn checkpoint(&self, _ctx: &Context<'_>, _id: usize) -> Option<Checkpoint> {
        None
    }
}

pub struct Route(RouteModel);

#[Object]
impl Route {
    async fn id(&self) -> ModelId<RouteId> {
        ModelId::from(self.0.id())
    }
    async fn name(&self) -> &str {
        &self.0.route.name
    }
    async fn distance(&self) -> f64 {
        self.0.route.distance
    }
    async fn geojson(&self) -> String {
        let linestring = geo::LineString::from(self.0.iter_geo_points().collect::<Vec<_>>());
        geojson::Feature::from(geojson::Geometry::try_from(&linestring).unwrap()).to_string()
    }
    async fn points(&self) -> Vec<Vec<f64>> {
        self.0
            .iter_geo_points()
            .map(|point| vec![point.x(), point.y()])
            .collect()
    }
    async fn polyline(&self) -> String {
        polyline::encode_coordinates(
            self.0
                .iter_geo_points()
                .flat_map(|point| point.coords_iter()),
            5,
        )
        .unwrap()
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
    async fn geojson<'ctx>(&self, ctx: &Context<'ctx>) -> Result<String, async_graphql::Error> {
        let ride_repo: &RideModelRepo = ctx.data()?;
        let ride_model = ride_repo
            .get(self.0.id)
            .await?
            .ok_or(anyhow::anyhow!("couldnt load model"))?;

        let linestring = geo::LineString::from_iter(ride_model.iter_geo_points());

        Ok(geojson::Feature::from(geojson::Geometry::try_from(&linestring).unwrap()).to_string())
    }
    async fn points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        let ride_repo: &RideModelRepo = ctx.data()?;
        let ride_model = ride_repo
            .get(self.0.id)
            .await?
            .ok_or(anyhow::anyhow!("couldnt load model"))?;

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
