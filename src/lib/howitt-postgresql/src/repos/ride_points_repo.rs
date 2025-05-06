use howitt::{
    ext::iter::ResultIterExt,
    models::ride::{RideId, RidePoints, RidePointsFilter},
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
            id: RideId::from(row.ride_id),
            points: serde_json::from_value(row.points)?,
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresRidePointsRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresRidePointsRepo {
    type Model = RidePoints;
    type Error = PostgresRepoError;

    async fn filter_models(
        &self,
        filter: RidePointsFilter,
    ) -> Result<Vec<RidePoints>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        match filter {
            RidePointsFilter::All => {
                let query = sqlx::query_as!(RidePointsRow, r#"select * from ride_points"#);

                Ok(query
                    .fetch_all(conn.as_mut())
                    .await?
                    .into_iter()
                    .map(RidePoints::try_from)
                    .collect_result_vec()?)
            }
            RidePointsFilter::Ids(ids) => {
                let uuids: Vec<_> = ids.into_iter().map(Uuid::from).collect();

                let query = sqlx::query_as!(
                    RidePointsRow,
                    r#"select * from ride_points where ride_id = ANY($1)"#,
                    &uuids
                );

                Ok(query
                    .fetch_all(conn.as_mut())
                    .await?
                    .into_iter()
                    .map(RidePoints::try_from)
                    .collect_result_vec()?)
            }
        }
    }

    async fn all(&self) -> Result<Vec<RidePoints>, PostgresRepoError> {
        self.filter_models(RidePointsFilter::All).await
    }

    async fn get(&self, id: RideId) -> Result<RidePoints, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RidePointsRow,
            r#"select * from ride_points where ride_id = $1"#,
            id.as_uuid()
        );

        Ok(RidePoints::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }

    async fn put(&self, ride_points: RidePoints) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query!(
            r#"insert into ride_points (
                ride_id,
                points
            ) values ($1, $2)
            ON CONFLICT (ride_id) DO UPDATE SET
                points = EXCLUDED.points"#,
            ride_points.id.as_uuid(),
            serde_json::to_value(ride_points.points)?
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
