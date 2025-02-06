use std::sync::Arc;

use crate::PostgresClient;
use howitt::repos::Repos;

mod media_repo;
mod poi_repo;
mod ride_points_repo;
mod ride_repo;
mod route_points_repo;
mod route_repo;
mod trip_repo;
mod user_repo;

pub use media_repo::PostgresMediaRepo;
pub use poi_repo::PostgresPointOfInterestRepo;
pub use ride_points_repo::PostgresRidePointsRepo;
pub use ride_repo::PostgresRideRepo;
pub use route_points_repo::PostgresRoutePointsRepo;
pub use route_repo::PostgresRouteRepo;
pub use trip_repo::PostgresTripRepo;
pub use user_repo::PostgresUserRepo;

#[derive(Clone)]
pub struct PostgresRepos {
    pub media_repo: PostgresMediaRepo,
    pub point_of_interest_repo: PostgresPointOfInterestRepo,
    pub ride_points_repo: PostgresRidePointsRepo,
    pub ride_repo: PostgresRideRepo,
    pub route_repo: PostgresRouteRepo,
    pub route_points_repo: PostgresRoutePointsRepo,
    pub trip_repo: PostgresTripRepo,
    pub user_repo: PostgresUserRepo,
}

impl PostgresRepos {
    pub fn new(client: PostgresClient) -> PostgresRepos {
        PostgresRepos {
            media_repo: PostgresMediaRepo::new(client.clone()),
            point_of_interest_repo: PostgresPointOfInterestRepo::new(client.clone()),
            ride_points_repo: PostgresRidePointsRepo::new(client.clone()),
            ride_repo: PostgresRideRepo::new(client.clone()),
            route_repo: PostgresRouteRepo::new(client.clone()),
            route_points_repo: PostgresRoutePointsRepo::new(client.clone()),
            trip_repo: PostgresTripRepo::new(client.clone()),
            user_repo: PostgresUserRepo::new(client.clone()),
        }
    }
}

impl From<PostgresRepos> for Repos {
    fn from(postgres_context: PostgresRepos) -> Self {
        Repos {
            media_repo: Arc::new(postgres_context.media_repo),
            point_of_interest_repo: Arc::new(postgres_context.point_of_interest_repo),
            ride_points_repo: Arc::new(postgres_context.ride_points_repo),
            ride_repo: Arc::new(postgres_context.ride_repo),
            route_repo: Arc::new(postgres_context.route_repo),
            route_points_repo: Arc::new(postgres_context.route_points_repo),
            trip_repo: Arc::new(postgres_context.trip_repo),
            user_repo: Arc::new(postgres_context.user_repo),
        }
    }
}
