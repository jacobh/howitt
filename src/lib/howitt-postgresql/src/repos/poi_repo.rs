use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::ext::serde::json::unwrap_string_value;

use howitt::models::point_of_interest::PointOfInterestId;
use howitt::models::user::UserId;
use howitt::models::{point_of_interest::PointOfInterest, Model};
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

#[allow(dead_code)]
struct PointOfInterestRow {
    id: Uuid,
    created_at: DateTime<Utc>,
    name: Option<String>,
    r#type: String,
    point: serde_json::Value,
    slug: String,
    user_id: Uuid,
    description: Option<String>,
}

impl TryFrom<PointOfInterestRow> for PointOfInterest {
    type Error = PostgresRepoError;

    fn try_from(row: PointOfInterestRow) -> Result<Self, Self::Error> {
        Ok(PointOfInterest {
            id: PointOfInterestId::from(row.id),
            user_id: UserId::from(row.user_id),
            name: row.name.unwrap_or_default(),
            slug: row.slug,
            point: serde_json::from_value(row.point)?,
            point_of_interest_type: serde_json::from_value(serde_json::Value::String(row.r#type))?,
            description: row.description,
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresPointOfInterestRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresPointOfInterestRepo {
    type Model = PointOfInterest;
    type Error = PostgresRepoError;

    async fn filter_models(&self, _filter: ()) -> Result<Vec<PointOfInterest>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(PointOfInterestRow, r#"select * from points_of_interest"#);

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(PointOfInterest::try_from)
            .collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<PointOfInterest>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(PointOfInterestRow, r#"select * from points_of_interest"#);

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(PointOfInterest::try_from)
            .collect_result_vec()?)
    }
    async fn get(
        &self,
        id: <PointOfInterest as Model>::Id,
    ) -> Result<PointOfInterest, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            PointOfInterestRow,
            r#"select * from points_of_interest where id = $1"#,
            id.as_uuid()
        );

        Ok(PointOfInterest::try_from(
            query.fetch_one(conn.as_mut()).await?,
        )?)
    }

    async fn put(&self, model: PointOfInterest) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query!(
            r#"insert into points_of_interest (
                id,
                created_at,
                name,
                type,
                point,
                user_id,
                slug,
                description
            ) values ($1, $2, $3, $4, $5, $6, $7, $8)
             on conflict (id) do update set 
                name = $3, 
                type = $4, 
                point = $5,
                user_id = $6,
                slug = $7,
                description = $8
             "#,
            model.id.as_uuid(),
            Utc::now(),
            model.name,
            unwrap_string_value(serde_json::to_value(model.point_of_interest_type)?),
            serde_json::to_value(model.point)?,
            model.user_id.as_uuid(),
            model.slug,
            model.description,
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
