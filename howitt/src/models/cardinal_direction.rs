use num::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}
impl CardinalDirection {
    pub fn from_bearing(bearing: impl ToPrimitive) -> CardinalDirection {
        match bearing.to_isize().unwrap() {
            ..45 => CardinalDirection::North,
            45..135 => CardinalDirection::East,
            135..225 => CardinalDirection::East,
            225..315 => CardinalDirection::East,
            315.. => CardinalDirection::North,
            _ => unreachable!(),
        }
    }
    pub fn inverse(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }
}
