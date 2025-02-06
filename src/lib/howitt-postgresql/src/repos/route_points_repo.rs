use howitt::{
    ext::iter::ResultIterExt,
    models::{
        route::{RouteId, RoutePoints},
        Model,
    },
    repos::Repo,
};
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

struct RoutePointsRow {
    route_id: Uuid,
    points: serde_json::Value,
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

    async fn filter_models(&self, _: ()) -> Result<Vec<RoutePoints>, PostgresRepoError> {
        self.all_indexes().await
    }

    async fn all_indexes(
        &self,
    ) -> Result<Vec<<RoutePoints as Model>::IndexItem>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(RoutePointsRow, r#"select * from route_points"#);

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(RoutePoints::try_from)
            .collect_result_vec()?)
    }

    async fn get(&self, id: RouteId) -> Result<RoutePoints, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RoutePointsRow,
            r#"select * from route_points where route_id = $1"#,
            id.as_uuid()
        );

        Ok(RoutePoints::try_from(
            query.fetch_one(conn.as_mut()).await?,
        )?)
    }

    async fn get_index(&self, id: RouteId) -> Result<RoutePoints, PostgresRepoError> {
        self.get(id).await
    }

    async fn put(&self, route_points: RoutePoints) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query!(
            r#"insert into route_points (
                route_id,
                points
            ) values ($1, $2)"#,
            route_points.id.as_uuid(),
            serde_json::to_value(route_points.points)?
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
