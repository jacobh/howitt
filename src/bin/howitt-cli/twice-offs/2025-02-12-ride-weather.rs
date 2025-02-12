use chrono::{DateTime, TimeZone, Timelike, Utc};
use howitt::{
    models::{
        point::{Point, WithDatetime, WithElevation},
        ride::RideId,
    },
    repos::AnyhowRepo,
};
use howitt_postgresql::PostgresRepos;
use open_meteo::{
    schema::{HistoricalWeatherParams, HourlyVariable},
    OpenMeteoHistoryClient,
};
use std::collections::HashMap;
use uuid::Uuid;

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
    // For testing, use a specific ride ID
    // 2024-12-15 10:41:28+11
    let ride_id = RideId::from(Uuid::parse_str("0193c9b2-a788-e44e-a072-a15bbd0be4a0")?);

    // Load ride and points
    let ride = ride_repo.get(ride_id).await?;
    let ride_points = ride_points_repo.get(ride_id).await?;

    println!(
        "Found {} points for ride {}",
        ride_points.points.len(),
        ride_id
    );

    // Group points by hour
    let mut hour_buckets: HashMap<DateTime<Utc>, Vec<_>> = HashMap::new();

    for point in ride_points.points {
        let hour = point
            .datetime()
            .with_minute(0)
            .and_then(|d| d.with_second(0))
            .and_then(|d| d.with_nanosecond(0))
            .unwrap();

        hour_buckets.entry(hour).or_default().push(point);
    }

    println!("Grouped into {} hour buckets", hour_buckets.len());

    // Take first point from each bucket
    let mut hourly_points: Vec<_> = hour_buckets
        .into_iter()
        .map(|(hour, points)| points.into_iter().next().unwrap())
        .collect();

    hourly_points.sort_by_key(|p| *p.datetime());

    // Initialize OpenMeteo client
    let client = OpenMeteoHistoryClient::new();

    // Process each point
    for point in &hourly_points {
        let params = HistoricalWeatherParams {
            start_date: point.datetime().date_naive().to_string(),
            end_date: point.datetime().date_naive().to_string(),
            hourly: Some(vec![
                HourlyVariable::Temperature2m,
                HourlyVariable::RelativeHumidity2m,
                HourlyVariable::WindSpeed10m,
                HourlyVariable::WindDirection10m,
            ]),
            daily: None,
            latitude: point.as_geo_point().y(),
            longitude: point.as_geo_point().x(),
            temperature_unit: None,
            wind_speed_unit: None,
            timeformat: None,
            timezone: Some("UTC".to_string()),
        };

        let weather = client.get_historical_weather(params).await?;

        // Find matching hour in response
        if let Some(hourly) = weather.hourly {
            for (i, time) in hourly.time.iter().enumerate() {
                let weather_time = chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M")
                    .unwrap()
                    .and_utc();
                if weather_time.hour() == point.datetime().hour() {
                    println!("Hour: {}", point.datetime());
                    println!(
                        "  Location: ({}, {})",
                        point.as_geo_point().y(),
                        point.as_geo_point().x()
                    );
                    println!("  Elevation: {}m", point.elevation());
                    println!(
                        "  Temperature: {}°C",
                        hourly
                            .temperature_2m
                            .as_ref()
                            .map(|t| t[i])
                            .unwrap_or_default()
                    );
                    println!(
                        "  Humidity: {}%",
                        hourly
                            .relative_humidity_2m
                            .as_ref()
                            .map(|h| h[i])
                            .unwrap_or_default()
                    );
                    println!(
                        "  Wind Speed: {} km/h",
                        hourly
                            .wind_speed_10m
                            .as_ref()
                            .map(|w| w[i])
                            .unwrap_or_default()
                    );
                    println!(
                        "  Wind Direction: {}°",
                        hourly
                            .wind_direction_10m
                            .as_ref()
                            .map(|d| d[i])
                            .unwrap_or_default()
                    );
                    println!();
                    break;
                }
            }
        }
    }

    Ok(())
}

// Found 34431 points for ride RIDE#0193c9b2-a788-e44e-a072-a15bbd0be4a0
// Grouped into 11 hour buckets
// Hour: 2024-12-14 23:41:28 UTC
//   Location: (-37.752811, 145.689606)
//   Elevation: 157.2m
//   Temperature: 20.9°C
//   Humidity: 75%
//   Wind Speed: 4.5 km/h
//   Wind Direction: 247°

// Hour: 2024-12-15 00:00:00 UTC
//   Location: (-37.754002, 145.731155)
//   Elevation: 180.2m
//   Temperature: 25.8°C
//   Humidity: 61%
//   Wind Speed: 4.5 km/h
//   Wind Direction: 236°

// Hour: 2024-12-15 01:00:00 UTC
//   Location: (-37.777287, 145.808014)
//   Elevation: 421.6m
//   Temperature: 26.2°C
//   Humidity: 46%
//   Wind Speed: 6 km/h
//   Wind Direction: 243°

// Hour: 2024-12-15 02:00:00 UTC
//   Location: (-37.775623, 145.825424)
//   Elevation: 644m
//   Temperature: 27.9°C
//   Humidity: 27%
//   Wind Speed: 6.9 km/h
//   Wind Direction: 223°

// Hour: 2024-12-15 03:00:00 UTC
//   Location: (-37.795219, 145.847229)
//   Elevation: 791.2m
//   Temperature: 26.6°C
//   Humidity: 35%
//   Wind Speed: 9.6 km/h
//   Wind Direction: 213°

// Hour: 2024-12-15 04:00:00 UTC
//   Location: (-37.805908, 145.900269)
//   Elevation: 805m
//   Temperature: 25.6°C
//   Humidity: 45%
//   Wind Speed: 10.8 km/h
//   Wind Direction: 201°

// Hour: 2024-12-15 05:00:00 UTC
//   Location: (-37.802364, 145.923431)
//   Elevation: 797.8m
//   Temperature: 25.3°C
//   Humidity: 48%
//   Wind Speed: 10.4 km/h
//   Wind Direction: 188°

// Hour: 2024-12-15 06:00:00 UTC
//   Location: (-37.764103, 145.93811)
//   Elevation: 720m
//   Temperature: 25.2°C
//   Humidity: 50%
//   Wind Speed: 9 km/h
//   Wind Direction: 167°

// Hour: 2024-12-15 07:00:00 UTC
//   Location: (-37.769978, 145.974808)
//   Elevation: 723.8m
//   Temperature: 24.4°C
//   Humidity: 52%
//   Wind Speed: 9.2 km/h
//   Wind Direction: 160°

// Hour: 2024-12-15 08:00:00 UTC
//   Location: (-37.766277, 145.996765)
//   Elevation: 844.2m
//   Temperature: 22.4°C
//   Humidity: 62%
//   Wind Speed: 7.3 km/h
//   Wind Direction: 123°

// Hour: 2024-12-15 09:00:00 UTC
//   Location: (-37.772995, 146.036774)
//   Elevation: 1055.2m
//   Temperature: 18.7°C
//   Humidity: 76%
//   Wind Speed: 8 km/h
//   Wind Direction: 80°
