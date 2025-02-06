use howitt_postgresql::PostgresRepos;

use crate::Context;

#[allow(unused_variables)]
pub async fn handle(
    Context {
        postgres_client,
        repos:
            PostgresRepos {
                user_repo,
                ride_repo,
                ride_points_repo,
                trip_repo,
                media_repo,
                route_repo,
                route_points_repo,
                point_of_interest_repo,
            },
        job_storage,
    }: Context,
) -> Result<(), anyhow::Error> {
    Ok(())
}
