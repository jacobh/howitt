use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::maybe_pair::MaybePair;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, derive_more::Display)]
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

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.millimeters().partial_cmp(&other.millimeters())
    }
}

impl Eq for Distance {}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn o_f(x: f64) -> ordered_float::OrderedFloat<f64> {
            ordered_float::OrderedFloat(x)
        }

        o_f(self.millimeters()).cmp(&o_f(other.millimeters()))
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum DifficultyRating {
    Green,
    Blue,
    Black,
    DoubleBlack,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum Rigid {
    Rigid,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, derive_more::Display)]
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

impl PartialOrd for SuspensionTravel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.millimeters().partial_cmp(&other.millimeters())
    }
}

impl Eq for SuspensionTravel {}

impl Ord for SuspensionTravel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn o_f(x: f64) -> ordered_float::OrderedFloat<f64> {
            ordered_float::OrderedFloat(x)
        }

        o_f(self.millimeters()).cmp(&o_f(other.millimeters()))
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum Scouted {
    Yes,
    Partially,
    No,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, derive_more::Display)]
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
    #[serde(default)]
    pub tags: Vec<String>,
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

    pub fn is_meta_complete(&self) -> bool {
        match self.scouted {
            Some(Scouted::Yes) | None => {
                self.technical_difficulty.is_some()
                    && self.physical_difficulty.is_some()
                    && self.minimum_bike.is_some()
                    && self.ideal_bike.is_some()
                    && self.direction.is_some()
                    && self.scouted.is_some()
            }
            Some(Scouted::Partially) | Some(Scouted::No) => true,
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

        assert!(result.is_ok())
    }
}
