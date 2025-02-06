use anyhow::anyhow;
use async_graphql::{Context, Enum, Object};
use howitt::{
    models::{
        media::MediaFilter,
        point::{
            delta::{BearingDelta, Delta, DistanceDelta, ElevationDelta},
            progress::{DistanceElevationProgress, DistanceProgress, Progress},
            ElevationPoint,
        },
        route::RouteId,
        tag::Tag,
    },
    services::generate_cuesheet::generate_cuesheet,
};
use itertools::Itertools;

use crate::graphql::context::SchemaData;

use crate::graphql::schema::{
    cue::Cue,
    external_ref::ExternalRef,
    geo::{PointDelta, SlopeEnd},
    user::UserProfile,
    ModelId,
};

use super::media::Media;

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

pub struct NearbyRoute {
    delta: PointDelta,
    closest_terminus_delta: PointDelta,
    closest_terminus: Terminus,
}

#[Object]
impl NearbyRoute {
    async fn delta(&self) -> &PointDelta {
        &self.delta
    }
    async fn closest_terminus_delta(&self) -> &PointDelta {
        &self.closest_terminus_delta
    }
    async fn closest_terminus(&self) -> &Terminus {
        &self.closest_terminus
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "howitt::models::terminus::TerminusEnd")]
pub enum TerminusEnd {
    Start,
    End,
}

pub struct Terminus {
    terminus: howitt::models::terminus::Terminus<ElevationPoint>,
    route: howitt::models::route::Route,
}

#[Object]
impl Terminus {
    async fn route(&self) -> Route {
        Route(self.route.clone())
    }
    async fn point(&self) -> Vec<f64> {
        let Terminus { terminus, .. } = self;

        let (x, y) = terminus.point().point.x_y();
        vec![x, y]
    }

    async fn end(&self) -> TerminusEnd {
        let Terminus { terminus, .. } = self;

        TerminusEnd::from(terminus.end)
    }

    async fn bearing(&self) -> f64 {
        let Terminus { terminus, .. } = self;

        terminus.bearing()
    }

    async fn distance_from_start(&self) -> f64 {
        let Terminus { terminus, .. } = self;

        terminus.distance_from_start()
    }

    async fn elevation_gain_from_start(&self) -> f64 {
        let Terminus { terminus, .. } = self;

        terminus.elevation().elevation_gain_from_start
    }

    async fn slope_end(&self) -> SlopeEnd {
        let Terminus { terminus, .. } = self;

        SlopeEnd::from(terminus.elevation().slope_end)
    }

    async fn nearby_routes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<NearbyRoute>, async_graphql::Error> {
        let Terminus { terminus, route } = self;

        let SchemaData { route_repo, .. } = ctx.data()?;

        let route_indexes = route_repo
            .all_indexes()
            .await?
            .into_iter()
            .filter(|route| route.tags.contains(&Tag::Starred))
            .collect_vec();

        Ok(route
            .routes_near_terminus(&route_indexes, terminus.end)
            .into_iter()
            .filter_map(|(_, route, closest_point, delta)| {
                let closest_terminus = route
                    .termini()
                    .map(|t| t.map_points(|p| p.clone()).closest_terminus(closest_point));

                closest_terminus.map(|closest_terminus| {
                    let point1 = terminus.point();
                    let point2 = closest_terminus.point();
                    let terminus_delta =
                        <(DistanceDelta, BearingDelta, ElevationDelta)>::delta(point1, point2);

                    NearbyRoute {
                        delta: PointDelta::from(delta),
                        closest_terminus_delta: PointDelta::from(terminus_delta),
                        closest_terminus: Terminus {
                            terminus: closest_terminus,
                            route: route.clone(),
                        },
                    }
                })
            })
            .collect_vec())
    }
}

pub struct Route(pub howitt::models::route::Route);

impl Route {
    fn route_description(&self) -> Option<&howitt::models::route_description::RouteDescription> {
        self.0.description.as_ref()
    }
}

#[Object]
impl Route {
    async fn id(&self) -> ModelId<RouteId> {
        ModelId(self.0.id())
    }
    async fn external_ref(&self) -> Option<ExternalRef> {
        self.0.external_ref.clone().map(ExternalRef)
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn slug(&self) -> &str {
        &self.0.slug
    }
    async fn distance(&self) -> f64 {
        self.0.distance
    }
    async fn elevation_ascent_m<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<f64, async_graphql::Error> {
        let SchemaData {
            route_points_loader,
            ..
        } = ctx.data()?;
        let route_points = route_points_loader
            .load_one(self.0.id())
            .await?
            .ok_or(anyhow!("Points not found"))?;

        // Convert points to progress and get final values for elevation gain
        let points = route_points.iter_elevation_points().cloned().collect_vec();
        let progress = DistanceElevationProgress::last_from_points(points)
            .map(|p| p.elevation_gain_m)
            .unwrap_or(0.0);

        Ok(progress)
    }

    async fn elevation_descent_m<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<f64, async_graphql::Error> {
        let SchemaData {
            route_points_loader,
            ..
        } = ctx.data()?;
        let route_points = route_points_loader
            .load_one(self.0.id())
            .await?
            .ok_or(anyhow!("Points not found"))?;

        // Convert points to progress and get final values for elevation loss
        let points = route_points.iter_elevation_points().cloned().collect_vec();
        let progress = DistanceElevationProgress::last_from_points(points)
            .map(|p| p.elevation_loss_m)
            .unwrap_or(0.0);

        Ok(progress)
    }
    async fn termini(&self) -> Vec<Terminus> {
        self.0
            .termini()
            .map(|t| t.map_points(|p| p.clone()).to_termini_vec())
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
    async fn tags(&self) -> Option<&Vec<String>> {
        Some(&self.route_description()?.tags)
    }
    async fn is_meta_complete(&self) -> bool {
        match self.route_description() {
            Some(description) => description.is_meta_complete(),
            None => false,
        }
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
    async fn sample_points_count(&self) -> usize {
        self.0
            .sample_points
            .as_ref()
            .map(|points| points.len())
            .unwrap_or(0)
    }
    async fn sample_points(&self) -> Vec<Vec<f64>> {
        self.0
            .sample_points
            .iter()
            .flatten()
            .map(howitt::models::point::Point::to_x_y_vec)
            .collect()
    }
    async fn points_count<'ctx>(&self, ctx: &Context<'ctx>) -> Result<usize, async_graphql::Error> {
        let SchemaData {
            route_points_loader,
            ..
        } = ctx.data()?;
        let route_points = route_points_loader
            .load_one(self.0.id())
            .await?
            .ok_or(anyhow!("Points not found"))?;

        Ok(route_points.iter_geo_points().count())
    }

    async fn points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        let SchemaData {
            route_points_loader,
            ..
        } = ctx.data()?;
        let route_points = route_points_loader
            .load_one(self.0.id())
            .await?
            .ok_or(anyhow!("Points not found"))?;

        Ok(route_points
            .iter_geo_points()
            .map(howitt::models::point::Point::into_x_y_vec)
            .collect())
    }
    async fn points_json<'ctx>(&self, ctx: &Context<'ctx>) -> Result<String, async_graphql::Error> {
        let points = self.points(ctx).await?;

        Ok(serde_json::to_string(&points)?)
    }
    pub async fn elevation_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData {
            route_points_loader,
            ..
        } = ctx.data()?;
        let route_points = route_points_loader
            .load_one(self.0.id())
            .await?
            .ok_or(anyhow!("Points not found"))?;

        // Get just the elevation values from each point
        Ok(route_points
            .iter_elevation_points()
            .map(|p| p.elevation)
            .collect())
    }

    pub async fn distance_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData {
            route_points_loader,
            ..
        } = ctx.data()?;
        let route_points = route_points_loader
            .load_one(self.0.id())
            .await?
            .ok_or(anyhow!("Points not found"))?;

        // Get elevation points and convert to distance progress
        let points = route_points.iter_elevation_points().cloned().collect_vec();
        let progress = DistanceProgress::from_points(points);

        // Extract just the distance values
        Ok(progress.into_iter().map(|p| p.distance_m).collect())
    }

    pub async fn elevation_points_json<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<String, async_graphql::Error> {
        Ok(serde_json::to_string(&self.elevation_points(ctx).await?)?)
    }
    pub async fn distance_points_json<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<String, async_graphql::Error> {
        Ok(serde_json::to_string(&self.distance_points(ctx).await?)?)
    }
    async fn cues<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Cue>, async_graphql::Error> {
        let SchemaData {
            route_points_loader,
            poi_repo,
            ..
        } = ctx.data()?;
        let route_points = route_points_loader
            .load_one(self.0.id())
            .await?
            .ok_or(anyhow!("Points not found"))?;

        let points = route_points.iter_elevation_points().cloned().collect_vec();
        let pois = poi_repo.all_indexes().await?;

        let cuesheet = generate_cuesheet(&points, &pois);

        Ok(cuesheet.cues.into_iter().map(Cue::from).collect_vec())
    }
    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserProfile, async_graphql::Error> {
        let SchemaData { user_repo, .. } = ctx.data()?;

        let user = user_repo.get(self.0.user_id).await?;

        Ok(UserProfile(user))
    }
    pub async fn media<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Media>, async_graphql::Error> {
        let SchemaData { media_repo, .. } = ctx.data()?;

        let media = media_repo
            .filter_models(MediaFilter::ForRoute(self.0.id()))
            .await?;

        Ok(media.into_iter().map(Media).collect())
    }
}
