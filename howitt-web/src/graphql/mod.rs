use async_graphql::*;
use howitt::config::Config;

pub struct Query;

#[Object]
impl Query {
    async fn routes<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Route>, async_graphql::Error> {
        let routes: &Vec<rwgps::types::Route> = ctx.data()?;
        Ok(routes
            .into_iter()
            .map(|route| Route(route.clone()))
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
            .map(|route| Route(route.clone()))
            .collect())
    }
    async fn route(&self, _ctx: &Context<'_>, _id: usize) -> Option<Route> {
        None
    }
    async fn checkpoints<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Checkpoint>, async_graphql::Error> {
        let checkpoints: &Vec<howitt::checkpoint::Checkpoint> = ctx.data()?;
        Ok(checkpoints
            .into_iter()
            .map(|checkpoint| Checkpoint(checkpoint.clone()))
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
        1
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn distance(&self) -> f64 {
        self.0.distance.unwrap_or(0.0)
    }
    async fn points(&self) -> Vec<Point> {
        self.0
            .track_points
            .clone()
            .into_iter()
            .map(geo::Point::from)
            .map(Point::from)
            .collect()
    }
}

pub struct Checkpoint(howitt::checkpoint::Checkpoint);

#[Object]
impl Checkpoint {
    async fn id(&self) -> usize {
        1
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn point(&self) -> Point {
        Point { lat: 0.0, lng: 0.0 }
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
