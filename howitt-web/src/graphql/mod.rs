use async_graphql::*;

pub struct Query;

#[Object]
impl Query {
    async fn routes(&self) -> Vec<Route> {
        vec![]
    }
    async fn route(&self, _ctx: &Context<'_>, _id: usize) -> Option<Route> {
        None
    }
    async fn checkpoints(&self) -> Vec<Checkpoint> {
        vec![]
    }
    async fn checkpoint(&self, _ctx: &Context<'_>, _id: usize) -> Option<Checkpoint> {
        None
    }
}


pub struct Route;

#[Object]
impl Route {
    async fn id(&self) -> usize {
        1
    }
    async fn points(&self) -> Vec<Point> {
        vec![]
    }
}

pub struct Checkpoint;

#[Object]
impl Checkpoint {
    async fn id(&self) -> usize {
        1
    }
    async fn point(&self) -> Point {
        Point {lat: 0.0, lng: 0.0}
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
    lng: f64
}