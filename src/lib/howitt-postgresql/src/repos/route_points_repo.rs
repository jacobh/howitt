use howitt::{
    ext::iter::ResultIterExt,
    models::route::{RouteId, RoutePoints, RoutePointsFilter},
    repos::Repo,
};
use tokio_postgres::types::Type;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

struct RoutePointsRow {
    route_id: Uuid,
    points: serde_json::Value,
}

impl TryFrom<&tokio_postgres::Row> for RoutePointsRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            route_id: row.try_get("route_id")?,
            points: row.try_get("points")?,
        })
    }
}

impl TryFrom<RoutePointsRow> for RoutePoints {
    type Error = PostgresRepoError;

    fn try_from(row: RoutePointsRow) -> Result<Self, Self::Error> {
        Ok(RoutePoints {
            id: RouteId::from(row.route_id),
            points: serde_json::from_value(row.points)?,
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresRoutePointsRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresRoutePointsRepo {
    type Model = RoutePoints;
    type Error = PostgresRepoError;

    async fn filter_models(
        &self,
        filter: RoutePointsFilter,
    ) -> Result<Vec<RoutePoints>, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        let route_points = match filter {
            RoutePointsFilter::Ids(ids) => {
                let uuids: Vec<_> = ids.into_iter().map(|id| id.as_uuid().clone()).collect();

                conn.query_typed(
                    r#"select * from route_points where route_id = ANY($1)"#,
                    &[(&uuids, Type::UUID_ARRAY)],
                )
                .await?
                .iter()
                .map(RoutePointsRow::try_from)
                .collect::<Result<Vec<_>, _>>()?
            }
        };

        Ok(route_points
            .into_iter()
            .map(RoutePoints::try_from)
            .collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<RoutePoints>, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        Ok(conn
            .query_typed(r#"select * from route_points"#, &[])
            .await?
            .iter()
            .map(RoutePointsRow::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(RoutePoints::try_from)
            .collect_result_vec()?)
    }

    async fn get(&self, id: RouteId) -> Result<RoutePoints, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        Ok(RoutePoints::try_from(RoutePointsRow::try_from(
            &conn
                .query_typed_one(
                    r#"select * from route_points where route_id = $1"#,
                    &[(&(id.as_uuid()), Type::UUID)],
                )
                .await?,
        )?)?)
    }

    async fn put(&self, route_points: RoutePoints) -> Result<(), PostgresRepoError> {
        let conn = self.client.acquire().await?;

        conn.execute_typed(
            r#"insert into route_points (
                route_id,
                points
            ) values ($1, $2)
            ON CONFLICT (route_id) DO UPDATE
            SET
                points = EXCLUDED.points"#,
            &[
                (route_points.id.as_uuid(), Type::UUID),
                (&serde_json::to_value(route_points.points)?, Type::JSONB),
            ],
        )
        .await?;

        Ok(())
    }
}
