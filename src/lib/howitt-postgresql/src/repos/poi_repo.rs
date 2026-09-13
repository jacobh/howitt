use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::ext::serde::json::unwrap_string_value;
use tokio_postgres::types::Type;

use howitt::models::point_of_interest::PointOfInterestId;
use howitt::models::user::UserId;
use howitt::models::{Model, point_of_interest::PointOfInterest};
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresPool, PostgresRepoError};

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

impl TryFrom<&tokio_postgres::Row> for PointOfInterestRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            r#type: row.try_get("type")?,
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            name: row.try_get("name")?,
            point: row.try_get("point")?,
            slug: row.try_get("slug")?,
            user_id: row.try_get("user_id")?,
            description: row.try_get("description")?,
        })
    }
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
    pool: PostgresPool,
}

#[async_trait::async_trait]
impl Repo for PostgresPointOfInterestRepo {
    type Model = PointOfInterest;
    type Error = PostgresRepoError;

    async fn filter_models(&self, _filter: ()) -> Result<Vec<PointOfInterest>, PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        Ok(conn
            .query_typed(r#"select * from points_of_interest"#, &[])
            .await?
            .iter()
            .map(PointOfInterestRow::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(PointOfInterest::try_from)
            .collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<PointOfInterest>, PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        Ok(conn
            .query_typed(r#"select * from points_of_interest"#, &[])
            .await?
            .iter()
            .map(PointOfInterestRow::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(PointOfInterest::try_from)
            .collect_result_vec()?)
    }
    async fn get(
        &self,
        id: <PointOfInterest as Model>::Id,
    ) -> Result<PointOfInterest, PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        Ok(PointOfInterest::try_from(PointOfInterestRow::try_from(
            &conn
                .query_typed_one(
                    r#"select * from points_of_interest where id = $1"#,
                    &[(&(id.as_uuid()), Type::UUID)],
                )
                .await?,
        )?)?)
    }

    async fn put(&self, model: PointOfInterest) -> Result<(), PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        conn.execute_typed(
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
            &[
                (model.id.as_uuid(), Type::UUID),
                (&Utc::now(), Type::TIMESTAMPTZ),
                (&model.name, Type::VARCHAR),
                (
                    &unwrap_string_value(serde_json::to_value(model.point_of_interest_type)?),
                    Type::VARCHAR,
                ),
                (&serde_json::to_value(model.point)?, Type::JSONB),
                (model.user_id.as_uuid(), Type::UUID),
                (&model.slug, Type::VARCHAR),
                (&model.description, Type::TEXT),
            ],
        )
        .await?;

        Ok(())
    }
}
