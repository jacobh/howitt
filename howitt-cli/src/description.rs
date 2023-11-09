use chrono::Utc;
use clap::Subcommand;
use howitt::models::{
    maybe_pair::MaybePair,
    route_description::{
        BikeSpec, DifficultyRating, Direction, Distance, Rigid, RouteDescription, Scouted,
        SuspensionTravel,
    },
};
use itertools::Itertools;

#[derive(Subcommand)]
pub enum Description {
    Generate,
}

pub async fn handle(command: &Description) -> Result<(), anyhow::Error> {
    match command {
        Description::Generate => generate_description(),
    }

    Ok(())
}

const DIFFICULTIES: [DifficultyRating; 4] = [
    DifficultyRating::Green,
    DifficultyRating::Blue,
    DifficultyRating::Black,
    DifficultyRating::DoubleBlack,
];

const TYRE_WIDTHS: [Distance; 6] = [
    Distance::Millimeters(32.0),
    Distance::Millimeters(40.0),
    Distance::Inches(2.0),
    Distance::Inches(2.2),
    Distance::Inches(2.4),
    Distance::Inches(2.6),
];

const SUSPENSION_TRAVELS: [SuspensionTravel; 6] = [
    SuspensionTravel::Rigid(Rigid::Rigid),
    SuspensionTravel::Travel(Distance::Millimeters(100.0)),
    SuspensionTravel::Travel(Distance::Millimeters(110.0)),
    SuspensionTravel::Travel(Distance::Millimeters(120.0)),
    SuspensionTravel::Travel(Distance::Millimeters(130.0)),
    SuspensionTravel::Travel(Distance::Millimeters(140.0)),
];

const SCOUTEDS: [Scouted; 3] = [Scouted::Yes, Scouted::Partially, Scouted::No];

const DIRECTIONS: [Direction; 3] = [
    Direction::Either,
    Direction::PrimarlityAsRouted,
    Direction::OnlyAsRouted,
];

pub fn generate_description() {
    fn bike_prompt(type_: &str, default: bool) -> Option<BikeSpec> {
        if !inquire::Confirm::new(&format!("Specify {type_} now?"))
            .with_default(default)
            .prompt()
            .unwrap()
        {
            return None;
        }

        let tyre_width_min = inquire::Select::new("Tyre width (min)", TYRE_WIDTHS.to_vec())
            .with_starting_cursor(2)
            .prompt()
            .unwrap();

        let tyre_width_max = inquire::Select::new(
            "Tyre width (max)",
            TYRE_WIDTHS
                .iter()
                .cloned()
                .filter(|width| width >= &tyre_width_min)
                .collect_vec(),
        )
        .prompt()
        .unwrap();

        let front_suspension_min =
            inquire::Select::new("Front suspension (min)", SUSPENSION_TRAVELS.to_vec())
                .prompt()
                .unwrap();

        let front_suspension_max = inquire::Select::new(
            "Front suspension (min)",
            SUSPENSION_TRAVELS
                .iter()
                .cloned()
                .filter(|travel| travel >= &front_suspension_min)
                .collect_vec(),
        )
        .prompt()
        .unwrap();

        let rear_suspension_min =
            inquire::Select::new("Rear suspension (min)", SUSPENSION_TRAVELS.to_vec())
                .prompt()
                .unwrap();

        let rear_suspension_max = inquire::Select::new(
            "Rear suspension (min)",
            SUSPENSION_TRAVELS
                .iter()
                .cloned()
                .filter(|travel| travel >= &rear_suspension_min)
                .collect_vec(),
        )
        .prompt()
        .unwrap();

        let tyre_width = MaybePair::from((tyre_width_min, tyre_width_max));
        let front_suspension = MaybePair::from((front_suspension_min, front_suspension_max));
        let rear_suspension = MaybePair::from((rear_suspension_min, rear_suspension_max));

        Some(BikeSpec {
            tyre_width,
            front_suspension,
            rear_suspension,
        })
    }

    let technical_difficulty = inquire::Select::new("Technical difficulty", DIFFICULTIES.to_vec())
        .with_starting_cursor(1)
        .prompt()
        .ok();
    let physical_difficulty = inquire::Select::new("Physical difficulty", DIFFICULTIES.to_vec())
        .with_starting_cursor(1)
        .prompt()
        .ok();
    let description = None;
    let minimum_bike = bike_prompt("minimum bike", true);
    let ideal_bike = bike_prompt("ideal bike", false);
    let scouted = inquire::Select::new("Scouted", SCOUTEDS.to_vec())
        .prompt()
        .ok();
    let direction = inquire::Select::new("Direction", DIRECTIONS.to_vec())
        .prompt()
        .ok();
    let published_at = if inquire::Confirm::new("Publish now?")
        .with_default(false)
        .prompt()
        .unwrap()
    {
        Some(Utc::now())
    } else {
        None
    };

    let tags = inquire::Text::new("Tags (comma separated)")
        .prompt()
        .unwrap()
        .split(",")
        .map(String::from)
        .collect_vec();

    let description = RouteDescription {
        technical_difficulty,
        physical_difficulty,
        description,
        published_at,
        minimum_bike,
        ideal_bike,
        scouted,
        direction,
        tags,
    };

    println!(
        "\n\n[backcountry_segment]\n{}",
        toml::to_string_pretty(&description).unwrap()
    );
}
