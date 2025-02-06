use howitt::{
    models::{
        external_ref::{ExternalId, ExternalRef, RwgpsId},
        point::ElevationPoint,
        route::{Route, RouteFilter, RouteId},
        user::UserRwgpsConnection,
    },
    repos::{RoutePointsRepo, RouteRepo},
    services::{
        simplify_points::{simplify_points, SimplifyTarget},
        slug::generate_slug,
    },
};
use rwgps_types::{client::AuthenticatedRwgpsClient, credentials::Credentials};

pub struct SyncRouteParams<RwgpsClient> {
    pub client: RwgpsClient,
    pub route_repo: RouteRepo,
    pub route_points_repo: RoutePointsRepo,
    pub rwgps_route_id: usize,
    pub connection: UserRwgpsConnection,
}

pub async fn sync_route<RwgpsClient: rwgps_types::client::RwgpsClient>(
    SyncRouteParams {
        client,
        rwgps_route_id,
        connection,
        route_repo,
        route_points_repo,
    }: SyncRouteParams<RwgpsClient>,
) -> Result<(), anyhow::Error> {
    // Check for existing route
    let existing_route = route_repo
        .filter_models(RouteFilter::RwgpsId(rwgps_route_id))
        .await?
        .into_iter()
        .next();

    // Create authenticated client
    let auth_client = client.with_credentials(Credentials::from_token(connection.access_token));

    // Fetch route details from RWGPS
    let rwgps_route = auth_client.route(rwgps_route_id).await?;

    // Convert track points to ElevationPoints
    let points = rwgps_route
        .track_points
        .into_iter()
        .filter_map(|track_point| {
            match (
                geo::Point::try_from(track_point.clone()),
                track_point.elevation,
            ) {
                (Ok(point), Some(elevation)) => Some(ElevationPoint { point, elevation }),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    let sample_points = simplify_points(&points, SimplifyTarget::TotalPoints(50));

    match existing_route {
        Some(mut existing_route) => {
            // Update only points and timestamps for existing route
            existing_route.external_ref = Some(ExternalRef {
                id: ExternalId::Rwgps(RwgpsId::Route(rwgps_route_id)),
                sync_version: Some(2),
                updated_at: rwgps_route.updated_at,
            });
            existing_route.sample_points = Some(sample_points);

            // Update route and points
            route_repo.put(existing_route.clone()).await?;
            route_points_repo
                .put(howitt::models::route::RoutePoints {
                    id: existing_route.id,
                    points,
                })
                .await?;
        }
        None => {
            // Create new route
            let id = RouteId::new();
            let route = Route {
                id,
                name: rwgps_route.name.clone(),
                slug: generate_slug(&rwgps_route.name),
                user_id: connection.user_id,
                distance: rwgps_route.distance.unwrap_or(0.0),
                sample_points: Some(sample_points),
                description: None,
                external_ref: Some(ExternalRef {
                    id: ExternalId::Rwgps(RwgpsId::Route(rwgps_route_id)),
                    sync_version: Some(2),
                    updated_at: rwgps_route.updated_at,
                }),
                tags: Default::default(),
            };

            // Save new route and points
            route_repo.put(route).await?;
            route_points_repo
                .put(howitt::models::route::RoutePoints { id, points })
                .await?;
        }
    }

    Ok(())
}
