use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePair<T> {
    Singleton(T),
    Pair(T, T),
}

impl<T> MaybePair<T> {
    pub fn map<U, F>(self, f: F) -> MaybePair<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            MaybePair::Singleton(a) => MaybePair::Singleton(f(a)),
            MaybePair::Pair(a, b) => MaybePair::Pair(f(a), f(b)),
        }
    }
}

impl<T> MaybePair<T>
where
    T: Clone,
{
    pub fn into_tuple(self) -> (T, T) {
        match self {
            MaybePair::Singleton(a) => (a.clone(), a),
            MaybePair::Pair(a, b) => (a, b),
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        let (a, b) = self.into_tuple();
        vec![a, b]
    }
}

impl<T> Default for MaybePair<T>
where
    T: Default,
{
    fn default() -> Self {
        MaybePair::Singleton(T::default())
    }
}

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
    pub tyre_width_mm: MaybePair<Distance>,
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
    pub technical_difficulty: Option<DifficultyRating>,
    pub physical_difficulty: Option<DifficultyRating>,
    pub minimum_bike: Option<BikeSpec>,
    pub ideal_bike: Option<BikeSpec>,
    pub scouted: Option<Scouted>,
    pub direction: Option<Direction>,
}

impl RouteDescription {
    pub fn parse(input: String) -> Result<RouteDescription, ()> {
        let parts = input.split("[backcountry_segment]").collect_vec();
        let description = parts.get(0);
        let toml_text = parts.get(1);

        match (description, toml_text) {
            (Some(description), Some(toml_text)) => {
                let mut table: toml::Table = toml::from_str(*toml_text).map_err(|_| ())?;
                table.insert("description".to_owned(), toml::Value::from(*description));

                Ok(table.try_into().map_err(|_| ())?)
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Wrapper {
    backcountry_segment: RouteDescription,
}
