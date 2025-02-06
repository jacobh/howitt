use chrono::Utc;
use clap::Subcommand;
use derive_more::derive::{Display, From, Into};
use howitt::{
    models::{
        ride::{Ride, RideFilter},
        trip::{Trip, TripFilter, TripId},
        user::User,
    },
    repos::Repo,
    services::slug::generate_slug,
};
use howitt_postgresql::PostgresRepos;
use inquire::{MultiSelect, Select, Text};
use itertools::Itertools;
use prettytable::{row, Table};

use crate::Context;

#[derive(Subcommand)]
pub enum TripCommands {
    List,
    Create,
}

pub async fn handle(
    command: &TripCommands,
    Context {
        repos:
            PostgresRepos {
                user_repo,
                ride_repo,
                trip_repo,
                ..
            },
        ..
    }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        TripCommands::List => {
            let trips = trip_repo.filter_models(TripFilter::All).await?;

            let mut table = Table::new();
            table.add_row(row!["id", "name", "year", "rides"]);

            for trip in trips {
                table.add_row(row![trip.id, trip.name, trip.year, trip.ride_ids.len()]);
            }

            table.printstd();
            Ok(())
        }
        TripCommands::Create => {
            #[derive(Display, From, Into)]
            #[display("{}", _0.username)]
            struct UserOption(User);

            #[derive(Display, From, Into)]
            #[display("{} ({})", _0.started_at.format("%Y-%m-%d"), _0.name)]
            struct RideOption(Ride);

            // Get all users to select from
            let users = user_repo.all().await?;

            // Have user select which account to create the trip for
            let UserOption(user) = Select::new(
                "Select user:",
                users.into_iter().map(UserOption).collect_vec(),
            )
            .prompt()?;

            let rides = ride_repo
                .filter_models(RideFilter::ForUser {
                    user_id: user.id,
                    started_at: None,
                })
                .await?
                .into_iter()
                .sorted_by_key(|ride| -ride.started_at.timestamp())
                .collect_vec();

            // Have user select multiple rides to include in the trip
            let selected_rides = MultiSelect::new(
                "Select rides to include:",
                rides.into_iter().map(RideOption).collect_vec(),
            )
            .prompt()?
            .into_iter()
            .map(Ride::from);

            // Get trip details
            let name = Text::new("Trip name:").prompt()?;
            let year = Text::new("Trip year:")
                .with_validator(|input: &str| match input.parse::<i32>() {
                    Ok(_) => Ok(inquire::validator::Validation::Valid),
                    Err(_) => Ok(inquire::validator::Validation::Invalid(
                        "Please enter a valid year".into(),
                    )),
                })
                .prompt()?
                .parse::<i32>()?;

            let description = Text::new("Description (optional):").prompt_skippable()?;

            // Create the trip
            let trip = Trip {
                id: TripId::new(),
                name: name.clone(),
                user_id: user.id,
                created_at: Utc::now(),
                year,
                description,
                slug: generate_slug(&name),
                ride_ids: selected_rides.into_iter().map(|r| r.id).collect(),
                media_ids: vec![],
                notes: vec![],
            };

            trip_repo.put(trip).await?;
            println!("Trip created successfully!");

            Ok(())
        }
    }
}
