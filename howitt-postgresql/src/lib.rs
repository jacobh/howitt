use std::collections::HashSet;

use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::ext::ulid::uuid_into_ulid;
use howitt::models::route::Route;
use howitt::models::route_description::RouteDescription;
use sqlx::PgPool;

use howitt::models::{route::RouteModel, Model};
use howitt::repos::Repo;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[error("Postgres Repo Error")]
pub enum PostgresRepoError {
    Sqlx(#[from] sqlx::Error),
    SerdeJson(#[from] serde_json::Error),
}

pub struct PostgresRouteRepo {
    pool: PgPool,
}

#[async_trait::async_trait]
impl Repo for PostgresRouteRepo {
    type Model = RouteModel;
    type Error = PostgresRepoError;

    async fn all_indexes(
        &self,
    ) -> Result<Vec<<RouteModel as Model>::IndexItem>, PostgresRepoError> {
        let mut conn = self.pool.acquire().await.unwrap();

        struct RouteIndexRow {
            id: Uuid,
            name: String,
            external_ref: Option<serde_json::Value>,
            distance_m: i32,
            sample_points: serde_json::Value,
            description: Option<String>,
            published_at: Option<DateTime<Utc>>,
            technical_difficulty: Option<String>,
            physical_difficulty: Option<String>,
            minimum_bike: Option<serde_json::Value>,
            ideal_bike: Option<serde_json::Value>,
            scouted: Option<String>,
            direction: Option<String>,
            tags: Vec<String>,
        }

        let query = sqlx::query_as!(
            RouteIndexRow,
            r#"select id, name, external_ref, distance_m, sample_points, description, published_at, technical_difficulty, physical_difficulty, minimum_bike, ideal_bike, scouted, direction, tags from routes"#
        );

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(|row| -> Result<Route, PostgresRepoError> {
                Ok(Route {
                    id: sqlx::Either::Left(uuid_into_ulid(row.id)),
                    name: row.name,
                    distance: row.distance_m as f64,
                    sample_points: Some(serde_json::from_value(row.sample_points)?),
                    description: Some(RouteDescription {
                        description: row.description,
                        published_at: row.published_at,
                        technical_difficulty: row
                            .technical_difficulty
                            .map(serde_json::Value::String)
                            .map(serde_json::from_value)
                            .transpose()?,
                        physical_difficulty: row
                            .physical_difficulty
                            .map(serde_json::Value::String)
                            .map(serde_json::from_value)
                            .transpose()?,
                        minimum_bike: row.minimum_bike.map(serde_json::from_value).transpose()?,
                        ideal_bike: row.ideal_bike.map(serde_json::from_value).transpose()?,
                        scouted: row
                            .scouted
                            .map(serde_json::Value::String)
                            .map(serde_json::from_value)
                            .transpose()?,
                        direction: row
                            .direction
                            .map(serde_json::Value::String)
                            .map(serde_json::from_value)
                            .transpose()?,
                        tags: row.tags,
                    }),
                    external_ref: row.external_ref.map(serde_json::from_value).transpose()?,
                    tags: HashSet::default(),
                })
            })
            .collect_result_vec()?)
    }
    async fn get(&self, _id: <RouteModel as Model>::Id) -> Result<RouteModel, PostgresRepoError> {
        unimplemented!()
    }
    async fn get_index(
        &self,
        _id: <RouteModel as Model>::Id,
    ) -> Result<<RouteModel as Model>::IndexItem, PostgresRepoError> {
        unimplemented!()
    }
    async fn put(&self, _model: RouteModel) -> Result<(), PostgresRepoError> {
        unimplemented!()
    }
}
