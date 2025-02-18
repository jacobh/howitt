use async_graphql::*;
use howitt::models::route::{RouteFilter, RouteId};
use howitt::models::tag::Tag;
use howitt::models::trip::{TripFilter, TripId};
use howitt::models::user::UserFilter;
use howitt::repos::Repos;
use itertools::Itertools;

use crate::graphql::context::{RequestData, SchemaData};
use crate::graphql::schema::ModelId;

use super::point_of_interest::PointOfInterest;
use super::ride::Ride;
use super::route::Route;
use super::trip::Trip;
use super::user::UserProfile;
use super::viewer::Viewer;

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

        let has_all_tags_passes = self.has_all_tags.clone().is_none_or(|tags| {
            tags.into_iter()
                .map(Tag::Custom)
                .all(|required_tag| route.tags.contains(&required_tag))
        });

        let has_some_tags_passes = self.has_some_tags.clone().is_none_or(|tags| {
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
        let SchemaData {
            repos: Repos { route_repo, .. },
            ..
        } = ctx.data()?;

        let routes = route_repo.filter_models(RouteFilter::Starred).await?;

        Ok(routes.into_iter().map(Route).collect())
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
                    .all(|filter| filter.route_is_selected(&route.0))
            })
            .collect_vec())
    }

    async fn route<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: ModelId<RouteId>,
    ) -> Result<Option<Route>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { route_repo, .. },
            ..
        } = ctx.data()?;

        let route = route_repo.get(id.0).await?;

        Ok(Some(Route(route)))
    }

    async fn trip<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: ModelId<TripId>,
    ) -> Result<Option<Trip>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { trip_repo, .. },
            ..
        } = ctx.data()?;

        let trip = trip_repo.get(id.0).await?;

        Ok(Some(Trip(trip)))
    }

    async fn trips<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Trip>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { trip_repo, .. },
            ..
        } = ctx.data()?;

        let trips = trip_repo.all().await?;

        Ok(trips
            .into_iter()
            .sorted_by_key(|trip| trip.created_at)
            .map(Trip)
            .collect())
    }

    async fn published_trips<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Trip>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { trip_repo, .. },
            ..
        } = ctx.data()?;

        let trips = trip_repo.filter_models(TripFilter::Published).await?;

        Ok(trips
            .into_iter()
            .sorted_by_key(|trip| trip.created_at)
            .map(Trip)
            .collect())
    }

    async fn route_with_slug<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
    ) -> Result<Option<Route>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { route_repo, .. },
            ..
        } = ctx.data()?;

        let route = route_repo.find_model(RouteFilter::Slug(slug)).await?;

        Ok(route.map(|r| Route(r)))
    }

    async fn rides<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { ride_repo, .. },
            ..
        } = ctx.data()?;
        let rides = ride_repo.all().await?;

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
        let SchemaData {
            repos: Repos {
                point_of_interest_repo,
                ..
            },
            ..
        } = ctx.data()?;

        let pois = point_of_interest_repo.all().await?;

        Ok(pois.into_iter().map(PointOfInterest).collect())
    }

    async fn point_of_interest_with_slug<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
    ) -> Result<Option<PointOfInterest>, async_graphql::Error> {
        Ok(None)
    }

    async fn user_with_username<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<Option<UserProfile>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { user_repo, .. },
            ..
        } = ctx.data()?;

        let user = user_repo.find_model(UserFilter::Username(username)).await?;

        Ok(user.map(UserProfile))
    }

    async fn public_users<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<UserProfile>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { user_repo, .. },
            ..
        } = ctx.data()?;

        let users = user_repo.all().await?;

        Ok(users.into_iter().map(UserProfile).collect_vec())
    }
}
