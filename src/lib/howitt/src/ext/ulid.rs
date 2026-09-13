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
    ulid::Ulid::from_bytes(uuid.into_bytes())
}

pub fn ulid_into_uuid(ulid: ulid::Ulid) -> Uuid {
    Uuid::from_bytes(ulid.to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_ulid_is_deterministic() {
        let datetime = Utc.with_ymd_and_hms(2024, 2, 3, 4, 5, 6).unwrap();
        let ulid = generate_ulid(Some(datetime), ("route", 42_u64)).unwrap();

        assert_eq!(ulid.to_string(), "01HNPJ8DAG7VNGGJJS2AY65AYA");
    }

    #[test]
    fn uuid_conversion_preserves_all_bits() {
        let uuid = Uuid::from_u128(0x0123456789abcdef_fedcba9876543210);

        assert_eq!(ulid_into_uuid(uuid_into_ulid(uuid)), uuid);
    }
}
