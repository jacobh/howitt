use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::maybe_pair::MaybePair;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Distance {
    #[serde(rename = "mm")]
    Millimeters(f64),
    #[serde(rename = "inches")]
    Inches(f64),
}

impl Distance {
    pub fn millimeters(&self) -> f64 {
        match self {
            Distance::Millimeters(mm) => *mm,
            Distance::Inches(inches) => inches * 25.4,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DifficultyRating {
    Green,
    Blue,
    Black,
    DoubleBlack,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rigid {
    Rigid,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuspensionTravel {
    Rigid(Rigid),
    Travel(Distance),
}

impl SuspensionTravel {
    pub fn millimeters(&self) -> f64 {
        match self {
            SuspensionTravel::Rigid(_) => 0.0,
            SuspensionTravel::Travel(x) => x.millimeters(),
        }
    }
}

impl Default for SuspensionTravel {
    fn default() -> Self {
        SuspensionTravel::Rigid(Rigid::Rigid)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BikeSpec {
    pub tyre_width: MaybePair<Distance>,
    #[serde(default)]
    pub front_suspension: MaybePair<SuspensionTravel>,
    #[serde(default)]
    pub rear_suspension: MaybePair<SuspensionTravel>,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scouted {
    Yes,
    Partially,
    No,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Either,
    PrimarlityAsRouted,
    OnlyAsRouted,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct RouteDescription {
    pub description: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub technical_difficulty: Option<DifficultyRating>,
    pub physical_difficulty: Option<DifficultyRating>,
    pub minimum_bike: Option<BikeSpec>,
    pub ideal_bike: Option<BikeSpec>,
    pub scouted: Option<Scouted>,
    pub direction: Option<Direction>,
}

impl RouteDescription {
    pub fn parse(input: String) -> Result<Option<RouteDescription>, DescriptionParseError> {
        let parts = input.split("[backcountry_segment]").collect_vec();

        let description = parts.first();
        let toml_text = parts.get(1);

        match (description, toml_text) {
            (Some(description), Some(toml_text)) => {
                let mut table: toml::Table = toml::from_str(toml_text)?;
                table.insert("description".to_owned(), toml::Value::from(*description));

                Ok(Some(table.try_into()?))
            }
            _ => Ok(None),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Description parse error")]
pub enum DescriptionParseError {
    Toml(#[from] toml::de::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DESCRIPTION: &str = r#"traverse through the heart of the vic high country

[backcountry_segment]
technical_difficulty = "blue"
physical_difficulty = "black"
scouted = "yes"
direction = "either"

[minimum_bike]
tyre_width = [{ inches = 2}, { inches = 2.2}]

[ideal_bike]
tyre_width = [{ inches = 2.4}, { inches = 2.6}]
front_suspension = [{mm = 100}, {mm=120}]
rear_suspension = ["rigid", {mm=100}]"#;

    #[test]
    fn it_parses_example() {
        let result = RouteDescription::parse(TEST_DESCRIPTION.to_owned());

        assert_eq!(result.is_ok(), true)
    }
}
