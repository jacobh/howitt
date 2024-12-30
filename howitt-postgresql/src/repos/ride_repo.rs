use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::ext::ulid::{ulid_into_uuid, uuid_into_ulid};
use howitt::models::point::PointChunk;
use howitt::models::ride::RideId;
use itertools::Itertools;

use howitt::models::{
    ride::{Ride, RideModel},
    Model,
};
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

struct RideRow {
    id: Uuid,
    name: Option<String>,
    created_at: DateTime<Utc>,
    external_ref: Option<serde_json::Value>,
    points: serde_json::Value,
    distance_m: i32,
    started_at: DateTime<Utc>,
    finished_at: DateTime<Utc>,
}

impl TryFrom<RideRow> for Ride {
    type Error = PostgresRepoError;

    fn try_from(row: RideRow) -> Result<Self, Self::Error> {
        Ok(Ride {
            id: RideId::from(uuid_into_ulid(row.id)),
            name: row.name.unwrap_or_default(),
            distance: row.distance_m as f64,
            external_ref: row.external_ref.map(serde_json::from_value).transpose()?,
            started_at: row.started_at,
            finished_at: row.finished_at,
        })
    }
}

impl TryFrom<RideRow> for RideModel {
    type Error = PostgresRepoError;

    fn try_from(row: RideRow) -> Result<Self, Self::Error> {
        let points = row.points.clone();
        let ride = Ride::try_from(row)?;
        let point_chunks = vec![PointChunk {
            model_id: ride.id(),
            idx: 0,
            points: serde_json::from_value(points)?,
        }];

        Ok(RideModel { ride, point_chunks })
    }
}

#[derive(Debug, derive_more::Constructor)]
pub struct PostgresRideRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresRideRepo {
    type Model = RideModel;
    type Error = PostgresRepoError;

    async fn filter_models(&self, _filter: ()) -> Result<Vec<RideModel>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(RideRow, r#"select * from rides"#);

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(RideModel::try_from)
            .collect_result_vec()?)
    }

    async fn all_indexes(&self) -> Result<Vec<<RideModel as Model>::IndexItem>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(RideRow, r#"select * from rides"#);

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(Ride::try_from)
            .collect_result_vec()?)
    }
    async fn get(&self, id: <RideModel as Model>::Id) -> Result<RideModel, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RideRow,
            r#"select * from rides where id = $1"#,
            ulid_into_uuid(*id.as_ulid())
        );

        Ok(RideModel::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }
    async fn get_index(
        &self,
        id: <RideModel as Model>::Id,
    ) -> Result<<RideModel as Model>::IndexItem, PostgresRepoError> {
        Ok(self.get(id).await?.ride)
    }
    async fn put(&self, model: RideModel) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let RideModel {
            ride, point_chunks, ..
        } = model;

        let points = PointChunk::into_iter_points(point_chunks).collect_vec();

        let query = sqlx::query!(
            r#"insert into rides (
                id,
                name,
                created_at,
                external_ref,
                points,
                distance_m,
                started_at,
                finished_at
            ) values ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            ulid_into_uuid(*ride.id().as_ulid()),
            ride.name,
            Utc::now(),
            ride.external_ref.map(serde_json::to_value).transpose()?,
            serde_json::to_value(points)?,
            ride.distance as i32,
            ride.started_at,
            ride.finished_at,
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
