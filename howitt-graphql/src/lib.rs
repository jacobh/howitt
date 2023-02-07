use async_graphql::*;
use howitt::{
    config::Config,
    repo::{CheckpointRepo, RouteRepo},
};

pub struct Query;

#[Object]
impl Query {
    async fn routes<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Route>, async_graphql::Error> {
        let route_repo: &RouteRepo = ctx.data()?;
        let routes = route_repo.all().await?;
        Ok(routes.into_iter().map(|route| Route(route)).collect())
    }
    async fn starred_routes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Route>, async_graphql::Error> {
        let config: &Config = ctx.data()?;
        let route_repo: &RouteRepo = ctx.data()?;
        let routes = route_repo.all().await?;
        Ok(routes.into_iter().filter(|route| config.starred_route_ids.contains(&route.id)).map(|route| Route(route)).collect())
    }
    async fn route(&self, _ctx: &Context<'_>, _id: usize) -> Option<Route> {
        None
    }
    async fn latest_rides(&self, ctx: &Context<'_>) -> Result<Vec<Ride>, async_graphql::Error> {
        let now = chrono::Utc::now();
        let thirty_days_ago = now - chrono::Duration::days(30);
        let trips: &Vec<rwgps::types::Trip> = ctx.data()?;

        Ok(trips
            .into_iter()
            .filter(|trip| trip.departed_at > thirty_days_ago)
            .cloned()
            .map(|trip| Ride(trip))
            .collect())
    }
    async fn checkpoints<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Checkpoint>, async_graphql::Error> {
        let checkpoint_repo: &CheckpointRepo = ctx.data()?;

        let x = checkpoint_repo.all().await?;

        Ok(x.clone()
            .into_iter()
            .map(|checkpoint| Checkpoint(checkpoint))
            .collect())
    }
    async fn checkpoint(&self, _ctx: &Context<'_>, _id: usize) -> Option<Checkpoint> {
        None
    }
}

pub struct Route(howitt::route::Route);

#[Object]
impl Route {
    async fn id(&self) -> String {
        self.0.id.to_string()
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn distance(&self) -> f64 {
        self.0.distance
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
}

pub struct Ride(rwgps::types::Trip);

#[Object]
impl Ride {
    async fn id(&self) -> usize {
        self.0.id
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn distance(&self) -> f64 {
        self.0.distance
    }
    async fn geojson(&self) -> String {
        let linestring = geo::LineString::from(self.0.clone());
        geojson::Feature::from(geojson::Geometry::try_from(&linestring).unwrap()).to_string()
    }
    async fn points(&self) -> Vec<Vec<f64>> {
        geo::LineString::from(self.0.clone())
            .into_points()
            .into_iter()
            .map(|point| vec![point.x(), point.y()])
            .collect()
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::checkpoint::CheckpointType")]
pub enum CheckpointType {
    RailwayStation,
    Hut,
    Locality,
    Generic,
}

pub struct Checkpoint(howitt::checkpoint::Checkpoint);

#[Object]
impl Checkpoint {
    async fn id(&self) -> &uuid::Uuid {
        &self.0.id
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
