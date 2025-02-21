use async_graphql::{Context, Object};
use chrono::{Duration, Utc};
use howitt::models::{ride::RideFilter, user::UserId};
use howitt::repos::Repos;
use itertools::Itertools;

use crate::graphql::context::{RequestData, SchemaData};
use crate::graphql::schema::{ride::Ride, route::Route, trip::Trip, IsoDate, ModelId};

use super::point_of_interest::PointOfInterest;

pub struct UserProfile(pub howitt::models::user::User);

#[Object]
impl UserProfile {
    async fn id(&self) -> ModelId<UserId> {
        ModelId::from(self.0.id)
    }

    async fn username(&self) -> &str {
        &self.0.username
    }

    async fn email<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Option<String>, async_graphql::Error> {
        let RequestData { login } = ctx.data()?;

        // Only return email if the viewer is the profile owner
        Ok(if let Some(login) = login {
            if login.session.user_id == self.0.id {
                Some(self.0.email.clone())
            } else {
                None
            }
        } else {
            None
        })
    }

    async fn rides<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { ride_repo, .. },
            ..
        } = ctx.data()?;

        let rides = ride_repo
            .filter_models(RideFilter::ForUser {
                user_id: self.0.id,
                started_at: None,
            })
            .await?;

        let rides = rides.into_iter().map(Ride).collect_vec();

        Ok(rides)
    }

    async fn recent_rides<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { ride_repo, .. },
            ..
        } = ctx.data()?;

        let rides = ride_repo
            .filter_models(RideFilter::ForUser {
                user_id: self.0.id,
                started_at: Some(howitt::models::filters::TemporalFilter::After {
                    after: Utc::now() - Duration::days(365),
                    first: None,
                }),
            })
            .await?;

        let rides = rides.into_iter().map(Ride).collect_vec();

        Ok(rides)
    }
    async fn trips<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Trip>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { trip_repo, .. },
            ..
        } = ctx.data()?;

        let trips = trip_repo
            .filter_models(howitt::models::trip::TripFilter::User(self.0.id))
            .await?;

        Ok(trips.into_iter().map(Trip).collect())
    }
    async fn rides_with_date<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        date: IsoDate,
    ) -> Result<Vec<Ride>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { ride_repo, .. },
            ..
        } = ctx.data()?;

        // Get rides for that user on that date
        let rides = ride_repo
            .filter_models(RideFilter::ForUserWithDate {
                user_id: self.0.id,
                date: date.0,
            })
            .await?;

        Ok(rides
            .into_iter()
            .sorted_by_key(|ride| ride.started_at)
            .map(Ride)
            .collect())
    }

    async fn trip_with_slug<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
    ) -> Result<Option<Trip>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { trip_repo, .. },
            ..
        } = ctx.data()?;

        let trip = trip_repo
            .find_model(howitt::models::trip::TripFilter::WithUserAndSlug {
                user_id: self.0.id,
                slug,
            })
            .await?;

        Ok(trip.map(Trip))
    }

    async fn routes<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Route>, async_graphql::Error> {
        let SchemaData {
            repos: Repos { route_repo, .. },
            ..
        } = ctx.data()?;

        let routes = route_repo
            .filter_models(howitt::models::route::RouteFilter::UserId(self.0.id))
            .await?;

        let routes = routes
            .into_iter()
            .map(crate::graphql::schema::route::Route)
            .collect_vec();
        Ok(routes)
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

        let points = point_of_interest_repo
            .all()
            .await?
            .into_iter()
            .filter(|poi| poi.user_id == self.0.id)
            .map(PointOfInterest)
            .collect();

        Ok(points)
    }
}
