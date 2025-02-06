use crate::Context;
use howitt::models::point::Point;
use howitt::models::ride::RideFilter;
use howitt::models::trip::TripId;
use howitt::models::Model;
use howitt::repos::AnyhowRepo;
use mapbox_geocoding::client::MapboxGeocodingClient;
use mapbox_geocoding::schema::ReverseGeocodingParams;
use uuid::Uuid;

const MAPBOX_ACCESS_TOKEN: &str = "..."; // Fill in your access token here

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
        media_repo,
        job_storage,
        route_points_repo,
    }: Context,
) -> Result<(), anyhow::Error> {
    // Get trip by ID
    let trip_id = TripId::from(Uuid::parse_str("01949892-a2a0-74e1-b9aa-bbf1dc7380ac")?);
    let trip = trip_repo.get(trip_id).await?;

    // Get all rides for this trip
    let rides = ride_repo
        .filter_models(RideFilter::ForTrip(trip.id))
        .await?;

    // Create Mapbox client
    let mapbox_client = MapboxGeocodingClient::new(MAPBOX_ACCESS_TOKEN.to_string());

    // For each ride
    for ride in rides {
        // Get points
        let points = ride_points_repo.get(ride.id).await?;

        // Calculate indices for 5 evenly distributed points
        let total_points = points.points.len();
        let sample_points = vec![points.points[0].clone()];
        // let sample_points: Vec<_> = if total_points >= 5 {
        //     (0..20)
        //         .map(|i| points.points[i * (total_points - 1) / 19].clone())
        //         .collect()
        // } else {
        //     points.points
        // };

        println!("Results for ride {}", ride.id.as_uuid());

        // Process each sample point
        for (i, point) in sample_points.iter().enumerate() {
            // Call reverse geocode
            let result = mapbox_client
                .reverse_geocode(ReverseGeocodingParams {
                    longitude: point.point.x(),
                    latitude: point.point.y(),
                    access_token: MAPBOX_ACCESS_TOKEN.to_string(),
                    permanent: None,
                    country: None,
                    language: None,
                    limit: None,
                    types: None,
                    worldview: None,
                })
                .await?;

            println!("Sample point {}: {:?}", i + 1, point.x_y());
            println!("Found {} features", result.features.len());

            // Print first feature if available
            if let Some(first_feature) = result.features.first() {
                println!("First feature: {}", first_feature.properties.name);
                if let Some(addr) = &first_feature.properties.full_address {
                    println!("Full address: {}", addr);
                }
                println!("---");
            }
        }
        println!("================");
    }

    Ok(())
}
