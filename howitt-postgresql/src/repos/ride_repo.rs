use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::models::filters::TemporalFilter;
use howitt::models::ride::{RideFilter, RideId};

use howitt::models::user::UserId;
use howitt::models::{ride::Ride, Model};
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

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

#[derive(Debug, derive_more::Constructor)]
pub struct PostgresRideRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresRideRepo {
    type Model = Ride;
    type Error = PostgresRepoError;

    async fn filter_models(&self, filter: RideFilter) -> Result<Vec<Ride>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let rides = match filter {
            RideFilter::ForUser {
                user_id,
                started_at: Some(TemporalFilter::Before {before, last}),
            } => {
                sqlx::query_as!(
                    RideRow,
                    r#"select * from rides where user_id = $1 and started_at < $2 order by started_at desc limit $3"#,
                    user_id.as_uuid(),
                    before,
                    last.unwrap_or(100_000) as i32
                )
                .fetch_all(conn.as_mut())
                .await
            }
            RideFilter::ForUser {
                user_id,
                started_at: Some(TemporalFilter::After {after, first}),
            } => {
                sqlx::query_as!(
                    RideRow,
                    r#"select * from rides where user_id = $1 and started_at > $2 order by started_at asc limit $3"#,
                    user_id.as_uuid(),
                    after,
                    first.unwrap_or(100_000) as i32
                )
                .fetch_all(conn.as_mut())
                .await
            }
            RideFilter::ForUser {
                user_id,
                started_at: None,
            } => {
                sqlx::query_as!(
                    RideRow,
                    r#"select * from rides where user_id = $1"#,
                    user_id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await
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

                sqlx::query_as!(
                    RideRow,
                    r#"select * from rides 
                    where user_id = $1 
                    and started_at >= $2
                    and started_at < $3
                    order by started_at asc"#,
                    user_id.as_uuid(),
                    start_of_day,
                    end_of_day
                )
                .fetch_all(conn.as_mut())
                .await
            }
            RideFilter::ForTrip(trip_id) => {
                sqlx::query_as!(
                    RideRow,
                    r#"
                    SELECT r.* 
                    FROM rides r
                    INNER JOIN trip_rides tr ON tr.ride_id = r.id
                    WHERE tr.trip_id = $1
                    ORDER BY r.started_at ASC
                    "#,
                    trip_id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await
            }
            RideFilter::All => {
                sqlx::query_as!(RideRow, r#"select * from rides"#)
                    .fetch_all(conn.as_mut())
                    .await
            }
        }?;

        Ok(rides.into_iter().map(Ride::try_from).collect_result_vec()?)
    }

    async fn all_indexes(&self) -> Result<Vec<<Ride as Model>::IndexItem>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(RideRow, r#"select * from rides"#);

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(Ride::try_from)
            .collect_result_vec()?)
    }
    async fn get(&self, id: <Ride as Model>::Id) -> Result<Ride, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RideRow,
            r#"select * from rides where id = $1"#,
            id.as_uuid(),
        );

        Ok(Ride::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }
    async fn get_index(
        &self,
        id: <Ride as Model>::Id,
    ) -> Result<<Ride as Model>::IndexItem, PostgresRepoError> {
        Ok(self.get(id).await?)
    }
    async fn put(&self, ride: Ride) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query!(
            r#"insert into rides (
                id,
                name,
                created_at,
                external_ref,
                distance_m,
                started_at,
                finished_at,
                user_id
            ) values ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            ride.id.as_uuid(),
            ride.name,
            Utc::now(),
            ride.external_ref.map(serde_json::to_value).transpose()?,
            ride.distance as i32,
            ride.started_at,
            ride.finished_at,
            ride.user_id.as_uuid(),
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
