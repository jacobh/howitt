use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

fn default_datetime() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap()
}

pub fn generate_ulid<TZ: chrono::TimeZone, T: serde::Serialize>(
    datetime: Option<DateTime<TZ>>,
    value: T,
) -> Result<ulid::Ulid, anyhow::Error> {
    let datetime = match datetime {
        Some(datetime) => datetime.with_timezone(&Utc),
        None => default_datetime(),
    };

    let value_bytes: Vec<u8> = bincode::serialize(&value)?;
    let value_digest = md5::compute(value_bytes);

    Ok(ulid::Ulid::from_parts(
        datetime.timestamp_millis() as u64,
        u128::from_le_bytes(*value_digest),
    ))
}

pub fn uuid_into_ulid(uuid: Uuid) -> ulid::Ulid {
    ulid::Ulid::from_bytes(uuid.to_bytes_le())
}
