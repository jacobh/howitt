use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::models::filters::TemporalFilter;
use howitt::models::ride::{RideFilter, RideId};
use tokio_postgres::types::Type;

use howitt::models::user::UserId;
use howitt::models::{Model, ride::Ride};
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresPool, PostgresRepoError};

#[allow(dead_code)]
struct RideRow {
    id: Uuid,
    name: Option<String>,
    created_at: DateTime<Utc>,
    external_ref: Option<serde_json::Value>,
    distance_m: i32,
    started_at: DateTime<Utc>,
    finished_at: DateTime<Utc>,
    user_id: Uuid,
}

impl TryFrom<&tokio_postgres::Row> for RideRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            created_at: row.try_get("created_at")?,
            external_ref: row.try_get("external_ref")?,
            distance_m: row.try_get("distance_m")?,
            started_at: row.try_get("started_at")?,
            finished_at: row.try_get("finished_at")?,
            user_id: row.try_get("user_id")?,
        })
    }
}

impl TryFrom<RideRow> for Ride {
    type Error = PostgresRepoError;

    fn try_from(row: RideRow) -> Result<Self, Self::Error> {
        Ok(Ride {
            id: RideId::from(row.id),
            name: row.name.unwrap_or_default(),
            user_id: UserId::from(row.user_id),
            distance: row.distance_m as f64,
            external_ref: row.external_ref.map(serde_json::from_value).transpose()?,
            started_at: row.started_at,
            finished_at: row.finished_at,
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresRideRepo {
    pool: PostgresPool,
}

#[async_trait::async_trait]
impl Repo for PostgresRideRepo {
    type Model = Ride;
    type Error = PostgresRepoError;

    async fn filter_models(&self, filter: RideFilter) -> Result<Vec<Ride>, PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        let rides = match filter {
            RideFilter::Ids(ids) => {
                let uuids: Vec<_> = ids.into_iter().map(Uuid::from).collect();

                conn.query_typed(
                    r#"select * from rides where id = ANY($1)"#,
                    &[(&uuids, Type::UUID_ARRAY)],
                ).await?.iter().map(RideRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RideFilter::ForUser {
                user_id,
                started_at: Some(TemporalFilter::Before {before, last}),
            } => {
                conn.query_typed(
                    r#"select * from rides where user_id = $1 and started_at < $2 order by started_at desc limit $3"#,
                    &[(user_id.as_uuid(), Type::UUID), (&before, Type::TIMESTAMPTZ), (&(last.unwrap_or(100_000) as i64), Type::INT8)],
                ).await?.iter().map(RideRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RideFilter::ForUser {
                user_id,
                started_at: Some(TemporalFilter::After {after, first}),
            } => {
                conn.query_typed(
                    r#"select * from rides where user_id = $1 and started_at > $2 order by started_at asc limit $3"#,
                    &[(user_id.as_uuid(), Type::UUID), (&after, Type::TIMESTAMPTZ), (&(first.unwrap_or(100_000) as i64), Type::INT8)],
                ).await?.iter().map(RideRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RideFilter::ForUser {
                user_id,
                started_at: None,
            } => {
                conn.query_typed(
                    r#"select * from rides where user_id = $1"#,
                    &[(user_id.as_uuid(), Type::UUID)],
                ).await?.iter().map(RideRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RideFilter::ForUserWithDate { user_id, date } => {
                // Convert naive date to UTC timestamps for the start and end of the day in Melbourne timezone
                let tz = chrono_tz::Australia::Melbourne;
                let start_of_day = date.and_hms_opt(0, 0, 0).unwrap()
                    .and_local_timezone(tz)
                    .unwrap()
                    .with_timezone(&Utc);
                let end_of_day = date.and_hms_opt(23, 59, 59).unwrap()
                    .and_local_timezone(tz)
                    .unwrap()
                    .with_timezone(&Utc);

                conn.query_typed(
                    r#"select * from rides
                    where user_id = $1
                    and started_at >= $2
                    and started_at < $3
                    order by started_at asc"#,
                    &[(user_id.as_uuid(), Type::UUID), (&start_of_day, Type::TIMESTAMPTZ), (&end_of_day, Type::TIMESTAMPTZ)],
                ).await?.iter().map(RideRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RideFilter::ForTrip(trip_id) => {
                conn.query_typed(
                    r#"
                    SELECT r.*
                    FROM rides r
                    INNER JOIN trip_rides tr ON tr.ride_id = r.id
                    WHERE tr.trip_id = $1
                    ORDER BY r.started_at ASC
                    "#,
                    &[(trip_id.as_uuid(), Type::UUID)],
                ).await?.iter().map(RideRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RideFilter::RwgpsId(rwgps_id) => {
                conn.query_typed(
                    r#"select * from rides where (external_ref->'id'->'Rwgps'->'Trip')::int = $1"#,
                    &[(&(rwgps_id as i32), Type::INT4)],
                ).await?.iter().map(RideRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RideFilter::All => {
                conn.query_typed(
                    r#"select * from rides"#,
                    &[],
                ).await?.iter().map(RideRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
        };

        Ok(rides.into_iter().map(Ride::try_from).collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<Ride>, PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        Ok(conn
            .query_typed(r#"select * from rides"#, &[])
            .await?
            .iter()
            .map(RideRow::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(Ride::try_from)
            .collect_result_vec()?)
    }
    async fn get(&self, id: <Ride as Model>::Id) -> Result<Ride, PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        Ok(Ride::try_from(RideRow::try_from(
            &conn
                .query_typed_one(
                    r#"select * from rides where id = $1"#,
                    &[(&(id.as_uuid()), Type::UUID)],
                )
                .await?,
        )?)?)
    }

    async fn put(&self, ride: Ride) -> Result<(), PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        conn.execute_typed(
            r#"insert into rides (
                id,
                name,
                created_at,
                external_ref,
                distance_m,
                started_at,
                finished_at,
                user_id
            ) values ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                external_ref = EXCLUDED.external_ref,
                distance_m = EXCLUDED.distance_m,
                started_at = EXCLUDED.started_at,
                finished_at = EXCLUDED.finished_at"#,
            &[
                (ride.id.as_uuid(), Type::UUID),
                (&ride.name, Type::VARCHAR),
                (&Utc::now(), Type::TIMESTAMPTZ),
                (
                    &ride.external_ref.map(serde_json::to_value).transpose()?,
                    Type::JSONB,
                ),
                (&(ride.distance as i32), Type::INT4),
                (&ride.started_at, Type::TIMESTAMPTZ),
                (&ride.finished_at, Type::TIMESTAMPTZ),
                (ride.user_id.as_uuid(), Type::UUID),
            ],
        )
        .await?;

        Ok(())
    }
}
