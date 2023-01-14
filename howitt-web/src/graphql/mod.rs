use async_graphql::*;
use howitt::config::Config;

pub struct Query;

#[Object]
impl Query {
    async fn routes<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Route>, async_graphql::Error> {
        let routes: &Vec<rwgps::types::Route> = ctx.data()?;
        Ok(routes
            .into_iter()
            .cloned()
            .map(|route| Route(route))
            .collect())
    }
    async fn starred_routes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Route>, async_graphql::Error> {
        let config: &Config = ctx.data()?;
        let routes: &Vec<rwgps::types::Route> = ctx.data()?;

        Ok(routes
            .into_iter()
            .filter(|route| config.starred_route_ids.contains(&route.id))
            .cloned()
            .map(|route| Route(route))
            .collect())
    }
    async fn route(&self, _ctx: &Context<'_>, _id: usize) -> Option<Route> {
        None
    }
    async fn rides(&self, ctx: &Context<'_>) -> Result<Vec<Ride>, async_graphql::Error> {
        let trips: &Vec<rwgps::types::Trip> = ctx.data()?;

        Ok(trips
            .into_iter()
            .cloned()
            .map(|trip| Ride(trip))
            .collect())
    }
    async fn checkpoints<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Checkpoint>, async_graphql::Error> {
        let checkpoints: &Vec<howitt::checkpoint::Checkpoint> = ctx.data()?;
        Ok(checkpoints
            .into_iter()
            .cloned()
            .map(|checkpoint| Checkpoint(checkpoint))
            .collect())
    }
    async fn checkpoint(&self, _ctx: &Context<'_>, _id: usize) -> Option<Checkpoint> {
        None
    }
}

pub struct Route(rwgps::types::Route);

#[Object]
impl Route {
    async fn id(&self) -> usize {
        self.0.id
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn distance(&self) -> f64 {
        self.0.distance.unwrap_or(0.0)
    }
    async fn geojson(&self) -> String {
        let linestring = geo::LineString::from(self.0.clone());
        geojson::Feature::from(geojson::Geometry::try_from(&linestring).unwrap()).to_string()
    }
    async fn points(&self) -> Vec<Vec<f64>> {
        self.0
            .track_points
            .clone()
            .into_iter()
            .map(geo::Point::try_from)
            .filter_map(Result::ok)
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
