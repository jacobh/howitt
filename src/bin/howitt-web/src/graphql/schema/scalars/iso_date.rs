use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IsoDate(pub NaiveDate);

#[Scalar]
impl ScalarType for IsoDate {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(IsoDate(NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(
                |e| {
                    InputValueError::custom(format!(
                        "Invalid date format, expected YYYY-MM-DD: {}",
                        e
                    ))
                },
            )?)),
            _ => Err(InputValueError::expected_type(Value::String(String::new()))),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.format("%Y-%m-%d").to_string())
    }
}
