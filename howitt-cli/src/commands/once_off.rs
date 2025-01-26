use crate::Context;

#[allow(unused_variables)]
pub async fn handle(
    Context {
        postgres_client,
        user_repo,
        route_repo,
        ride_repo,
        ride_points_repo,
        poi_repo,
        trip_repo,
    }: Context,
) -> Result<(), anyhow::Error> {
    Ok(())
}
