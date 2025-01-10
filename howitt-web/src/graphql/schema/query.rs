use async_graphql::*;
use howitt::models::route::{RouteFilter, RouteId};
use howitt::models::tag::Tag;
use howitt::models::ModelRef;
use itertools::Itertools;

use crate::graphql::context::{RequestData, SchemaData};

use super::point_of_interest::PointOfInterest;
use super::ride::Ride;
use super::route::Route;
use super::viewer::Viewer;
use super::ModelId;

#[derive(InputObject)]
pub struct QueryRouteFilters {
    is_published: Option<bool>,
    has_all_tags: Option<Vec<String>>,
    has_some_tags: Option<Vec<String>>,
}

impl QueryRouteFilters {
    fn route_is_selected(&self, route: &howitt::models::route::Route) -> bool {
        let is_published_passes = match self.is_published {
            Some(is_published) => is_published == route.published_at().is_some(),
            None => true,
        };

        let has_all_tags_passes = self.has_all_tags.clone().map_or(true, |tags| {
            tags.into_iter()
                .map(Tag::Custom)
                .all(|required_tag| route.tags.contains(&required_tag))
        });

        let has_some_tags_passes = self.has_some_tags.clone().map_or(true, |tags| {
            tags.into_iter()
                .map(Tag::Custom)
                .any(|required_tag| route.tags.contains(&required_tag))
        });

        is_published_passes && has_all_tags_passes && has_some_tags_passes
    }
}

#[derive(InputObject)]
pub struct QueryRoutesInput {
    filters: Vec<QueryRouteFilters>,
}

pub struct Query;

#[Object]
impl Query {
    async fn viewer<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Option<Viewer>, async_graphql::Error> {
        let RequestData { login } = ctx.data()?;

        match login {
            Some(login) => Ok(Some(Viewer(login.clone()))),
            None => Ok(None),
        }
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
        let SchemaData { route_repo, .. } = ctx.data()?;

        let routes = route_repo
            .filter_models(RouteFilter {
                is_starred: Some(true),
            })
            .await?;

        Ok(routes
            .into_iter()
            .map(ModelRef::from_model)
            .map(Route)
            .collect())
    }
    async fn query_routes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: QueryRoutesInput,
    ) -> Result<Vec<Route>, async_graphql::Error> {
        let routes = self.starred_routes(ctx).await?;

        Ok(routes
            .into_iter()
            .filter(|route| {
                input
                    .filters
                    .iter()
                    .all(|filter| filter.route_is_selected(route.0.as_index()))
            })
            .collect_vec())
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
