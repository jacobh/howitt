use howitt::{
    ext::{
        iter::ResultIterExt,
        ulid::{ulid_into_uuid, uuid_into_ulid},
    },
    models::{
        ride::{RideId, RidePoints},
        Model,
    },
    repos::Repo,
};
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

struct RidePointsRow {
    ride_id: Uuid,
    points: serde_json::Value,
}

impl TryFrom<RidePointsRow> for RidePoints {
    type Error = PostgresRepoError;

    fn try_from(row: RidePointsRow) -> Result<Self, Self::Error> {
        Ok(RidePoints {
            id: RideId::from(uuid_into_ulid(row.ride_id)),
            points: serde_json::from_value(row.points)?,
        })
    }
}

#[derive(Debug, derive_more::Constructor)]
pub struct PostgresRidePointsRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresRidePointsRepo {
    type Model = RidePoints;
    type Error = PostgresRepoError;

    async fn filter_models(&self, _: ()) -> Result<Vec<RidePoints>, PostgresRepoError> {
        self.all_indexes().await
    }

    async fn all_indexes(
        &self,
    ) -> Result<Vec<<RidePoints as Model>::IndexItem>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(RidePointsRow, r#"select * from ride_points"#);

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(RidePoints::try_from)
            .collect_result_vec()?)
    }
    async fn get(&self, id: RideId) -> Result<RidePoints, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RidePointsRow,
            r#"select * from ride_points where ride_id = $1"#,
            ulid_into_uuid(*id.as_ulid())
        );

        Ok(RidePoints::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }
    async fn get_index(&self, id: RideId) -> Result<RidePoints, PostgresRepoError> {
        self.get(id).await
    }
    async fn put(&self, ride_points: RidePoints) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query!(
            r#"insert into ride_points (
                ride_id,
                points
            ) values ($1, $2)"#,
            ulid_into_uuid(*ride_points.id.as_ulid()),
            serde_json::to_value(ride_points.points)?
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
