use async_graphql::{Context, Enum, Object};
use howitt::{
    models::{ModelRef, point::ElevationPoint, route::RouteId, tag::Tag},
    services::generate_cuesheet::generate_cuesheet,
};
use itertools::Itertools;

use crate::graphql::context::SchemaData;

use super::{
    ExternalRef, ModelId,
    cue::Cue,
    geo::{PointDelta, SlopeEnd},
    photo::Photo,
};

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
    route: ModelRef<howitt::models::route::RouteModel>,
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
            .as_index()
            .routes_near_terminus(&route_indexes, terminus.end)
            .into_iter()
            .filter_map(|(_, route, closest_point, delta)| {
                let closest_terminus = route
                    .termini()
                    .map(|t| t.map_points(|p| p.clone()).closest_terminus(closest_point));

                if let Some(closest_terminus) = closest_terminus {
                    Some(NearbyRoute {
                        delta: PointDelta::from(delta),
                        closest_terminus_delta: PointDelta::from(
                            howitt::models::point::PointDelta::from_points(
                                terminus.point(),
                                closest_terminus.point(),
                            ),
                        ),
                        closest_terminus: Terminus {
                            terminus: closest_terminus,
                            route: ModelRef::from_index(route.clone()),
                        },
                    })
                } else {
                    None
                }
            })
            .collect_vec())
    }
}

pub struct Route(pub ModelRef<howitt::models::route::RouteModel>);

impl Route {
    fn route_description(&self) -> Option<&howitt::models::route_description::RouteDescription> {
        self.0.as_index().description.as_ref()
    }
}

#[Object]
impl Route {
    async fn id(&self) -> ModelId<RouteId> {
        ModelId(self.0.id())
    }
    async fn external_ref(&self) -> Option<ExternalRef> {
        self.0.as_index().external_ref.clone().map(ExternalRef)
    }
    async fn name(&self) -> &str {
        &self.0.as_index().name
    }
    async fn distance(&self) -> f64 {
        self.0.as_index().distance
    }
    async fn elevation_ascent_m<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<f64, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.to_model(route_repo).await?;

        Ok(route_model.segment_summary().data.elevation_ascent_m)
    }
    async fn elevation_descent_m<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<f64, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.to_model(route_repo).await?.clone();

        Ok(route_model.segment_summary().data.elevation_descent_m)
    }
    async fn termini(&self) -> Vec<Terminus> {
        self.0
            .as_index()
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
    async fn photos<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Photo<RouteId>>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.to_model(route_repo).await?;

        Ok(route_model.photos.clone().into_iter().map(Photo).collect())
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
            .as_index()
            .sample_points
            .as_ref()
            .map(|points| points.len())
            .unwrap_or(0)
    }
    async fn sample_points(&self) -> Vec<Vec<f64>> {
        self.0
            .as_index()
            .sample_points
            .iter()
            .flatten()
            .map(howitt::models::point::Point::to_x_y_vec)
            .collect()
    }
    async fn points_count<'ctx>(&self, ctx: &Context<'ctx>) -> Result<usize, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.to_model(route_repo).await?;

        Ok(route_model.iter_geo_points().count())
    }
    async fn points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<Vec<f64>>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.to_model(route_repo).await?;

        Ok(route_model
            .iter_geo_points()
            .map(howitt::models::point::Point::into_x_y_vec)
            .collect())
    }
    async fn elevation_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.to_model(route_repo).await?;

        Ok(route_model
            .iter_elevation_points()
            .map(|point| point.elevation)
            .collect())
    }
    async fn distance_points<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<f64>, async_graphql::Error> {
        let SchemaData { route_repo, .. } = ctx.data()?;
        let route_model = self.0.to_model(route_repo).await?;

        Ok(route_model.iter_cum_distance().collect())
    }
    async fn cues<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Cue>, async_graphql::Error> {
        let SchemaData {
            route_repo,
            poi_repo,
            ..
        } = ctx.data()?;
        let route_model = route_repo.get(self.0.id()).await?;

        let points = route_model.iter_elevation_points().cloned().collect_vec();
        let pois = poi_repo.all_indexes().await?;

        let cuesheet = generate_cuesheet(&points, &pois);

        Ok(cuesheet.cues.into_iter().map(Cue::from).collect_vec())
    }
}
