use howitt::{
    ext::iter::ResultIterExt,
    models::ride::{RideId, RidePoints},
    repos::Repo,
};
use tokio_postgres::types::Type;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

struct RidePointsRow {
    ride_id: Uuid,
    points: serde_json::Value,
}

impl TryFrom<&tokio_postgres::Row> for RidePointsRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            ride_id: row.try_get("ride_id")?,
            points: row.try_get("points")?,
        })
    }
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

    async fn filter_models(&self, _: ()) -> Result<Vec<RidePoints>, PostgresRepoError> {
        self.all().await
    }

    async fn all(&self) -> Result<Vec<RidePoints>, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        Ok(conn
            .query_typed(r#"select * from ride_points"#, &[])
            .await?
            .iter()
            .map(RidePointsRow::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(RidePoints::try_from)
            .collect_result_vec()?)
    }
    async fn get(&self, id: RideId) -> Result<RidePoints, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        Ok(RidePoints::try_from(RidePointsRow::try_from(
            &conn
                .query_typed_one(
                    r#"select * from ride_points where ride_id = $1"#,
                    &[(&(id.as_uuid()), Type::UUID)],
                )
                .await?,
        )?)?)
    }

    async fn put(&self, ride_points: RidePoints) -> Result<(), PostgresRepoError> {
        let conn = self.client.acquire().await?;

        conn.execute_typed(
            r#"insert into ride_points (
                ride_id,
                points
            ) values ($1, $2)
            ON CONFLICT (ride_id) DO UPDATE SET
                points = EXCLUDED.points"#,
            &[
                (ride_points.id.as_uuid(), Type::UUID),
                (&serde_json::to_value(ride_points.points)?, Type::JSONB),
            ],
        )
        .await?;

        Ok(())
    }
}
