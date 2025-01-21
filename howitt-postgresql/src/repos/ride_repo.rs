use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::ext::ulid::{ulid_into_uuid, uuid_into_ulid};
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
            id: RideId::from(uuid_into_ulid(row.id)),
            name: row.name.unwrap_or_default(),
            user_id: UserId::from(uuid_into_ulid(row.id)),
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
            RideFilter::User {
                user_id,
                started_at: Some(TemporalFilter::Before {before, last}),
            } => {
                sqlx::query_as!(
                    RideRow,
                    r#"select * from rides where user_id = $1 and started_at < $2 order by started_at desc limit $3"#,
                    ulid_into_uuid(*user_id.as_ulid()),
                    before,
                    last.unwrap_or(100_000) as i32
                )
                .fetch_all(conn.as_mut())
                .await
            }
            RideFilter::User {
                user_id,
                started_at: Some(TemporalFilter::After {after, first}),
            } => {
                sqlx::query_as!(
                    RideRow,
                    r#"select * from rides where user_id = $1 and started_at > $2 order by started_at asc limit $3"#,
                    ulid_into_uuid(*user_id.as_ulid()),
                    after,
                    first.unwrap_or(100_000) as i32
                )
                .fetch_all(conn.as_mut())
                .await
            }
            RideFilter::User {
                user_id,
                started_at: None,
            } => {
                sqlx::query_as!(
                    RideRow,
                    r#"select * from rides where user_id = $1"#,
                    ulid_into_uuid(*user_id.as_ulid())
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
            ulid_into_uuid(*id.as_ulid())
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
            ulid_into_uuid(*ride.id().as_ulid()),
            ride.name,
            Utc::now(),
            ride.external_ref.map(serde_json::to_value).transpose()?,
            ride.distance as i32,
            ride.started_at,
            ride.finished_at,
            ulid_into_uuid(*ride.user_id.as_ulid()),
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
