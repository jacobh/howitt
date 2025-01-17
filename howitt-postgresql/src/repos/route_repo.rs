use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::ext::serde::json::unwrap_string_value;
use howitt::ext::ulid::{ulid_into_uuid, uuid_into_ulid};
use howitt::models::point::PointChunk;
use howitt::models::route::{Route, RouteFilter};
use howitt::models::route_description::RouteDescription;
use howitt::models::tag::Tag;
use itertools::Itertools;

use howitt::models::{route::RouteModel, Model};
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

#[allow(dead_code)]
struct RouteIndexRow {
    id: Uuid,
    created_at: DateTime<Utc>,
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
    is_starred: bool,
    user_id: Uuid,
}

impl TryFrom<RouteIndexRow> for Route {
    type Error = PostgresRepoError;

    fn try_from(row: RouteIndexRow) -> Result<Self, Self::Error> {
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
                tags: row.tags.clone(),
            }),
            external_ref: row.external_ref.map(serde_json::from_value).transpose()?,
            tags: std::iter::empty()
                .chain(row.tags.into_iter().map(Tag::Custom))
                .chain(
                    if row.is_starred {
                        vec![Tag::Starred]
                    } else {
                        vec![]
                    }
                    .into_iter(),
                )
                .into_iter()
                .collect(),
        })
    }
}

#[allow(dead_code)]
struct RouteRow {
    id: Uuid,
    created_at: DateTime<Utc>,
    name: String,
    external_ref: Option<serde_json::Value>,
    distance_m: i32,
    sample_points: serde_json::Value,
    points: serde_json::Value,
    description: Option<String>,
    published_at: Option<DateTime<Utc>>,
    technical_difficulty: Option<String>,
    physical_difficulty: Option<String>,
    minimum_bike: Option<serde_json::Value>,
    ideal_bike: Option<serde_json::Value>,
    scouted: Option<String>,
    direction: Option<String>,
    tags: Vec<String>,
    is_starred: bool,
    user_id: Uuid,
}

impl TryFrom<RouteRow> for Route {
    type Error = PostgresRepoError;

    fn try_from(row: RouteRow) -> Result<Self, Self::Error> {
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
                tags: row.tags.clone(),
            }),
            external_ref: row.external_ref.map(serde_json::from_value).transpose()?,
            tags: std::iter::empty()
                .chain(row.tags.into_iter().map(Tag::Custom))
                .chain(
                    if row.is_starred {
                        vec![Tag::Starred]
                    } else {
                        vec![]
                    }
                    .into_iter(),
                )
                .into_iter()
                .collect(),
        })
    }
}

impl TryFrom<RouteRow> for RouteModel {
    type Error = PostgresRepoError;

    fn try_from(row: RouteRow) -> Result<Self, Self::Error> {
        let points = row.points.clone();
        let route = Route::try_from(row)?;
        let point_chunks = vec![PointChunk {
            model_id: route.id(),
            idx: 0,
            points: serde_json::from_value(points)?,
        }];

        Ok(RouteModel::new(route, point_chunks, vec![]))
    }
}

#[derive(Debug, derive_more::Constructor)]
pub struct PostgresRouteRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresRouteRepo {
    type Model = RouteModel;
    type Error = PostgresRepoError;

    async fn filter_models(
        &self,
        filter: RouteFilter,
    ) -> Result<Vec<RouteModel>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let RouteFilter { is_starred } = filter;

        let query = sqlx::query_as!(
            RouteRow,
            r#"select * from routes where is_starred = $1 or is_starred is null"#,
            is_starred
        );

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(RouteModel::try_from)
            .collect_result_vec()?)
    }

    async fn all_indexes(
        &self,
    ) -> Result<Vec<<RouteModel as Model>::IndexItem>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RouteIndexRow,
            r#"select id,
                created_at,
                name,
                external_ref,
                distance_m,
                sample_points,
                description,
                published_at,
                technical_difficulty,
                physical_difficulty,
                minimum_bike,
                ideal_bike,
                scouted,
                direction,
                tags,
                is_starred,
                user_id
            from routes"#
        );

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(Route::try_from)
            .collect_result_vec()?)
    }
    async fn get(&self, id: <RouteModel as Model>::Id) -> Result<RouteModel, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RouteRow,
            r#"select * from routes where id = $1"#,
            ulid_into_uuid(*id.as_ulid())
        );

        Ok(RouteModel::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }
    async fn get_index(
        &self,
        id: <RouteModel as Model>::Id,
    ) -> Result<<RouteModel as Model>::IndexItem, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RouteIndexRow,
            r#"select
                id,
                created_at,
                name,
                external_ref,
                distance_m,
                sample_points,
                description,
                published_at,
                technical_difficulty,
                physical_difficulty,
                minimum_bike,
                ideal_bike,
                scouted,
                direction,
                tags,
                is_starred,
                user_id
            from routes where id = $1"#,
            Uuid::from(id)
        );

        Ok(Route::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }
    async fn put(&self, model: RouteModel) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let RouteModel {
            route,
            point_chunks,
            ..
        } = model;

        let points = PointChunk::into_iter_points(point_chunks).collect_vec();

        let query = sqlx::query!(
            r#"insert into routes (
                id,
                created_at,
                name,
                external_ref,
                sample_points,
                points,
                distance_m,
                description,
                published_at,
                technical_difficulty,
                physical_difficulty,
                minimum_bike,
                ideal_bike,
                scouted,
                direction,
                tags,
                is_starred
            ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"#,
            ulid_into_uuid(*route.id().as_ulid()),
            Utc::now(),
            route.name,
            route.external_ref.map(serde_json::to_value).transpose()?,
            route.sample_points.map(serde_json::to_value).transpose()?,
            serde_json::to_value(points)?,
            route.distance as i32,
            route
                .description
                .as_ref()
                .and_then(|x| x.description.clone()),
            route
                .description
                .as_ref()
                .and_then(|x| x.published_at.clone()),
            route
                .description
                .as_ref()
                .and_then(|x| x.technical_difficulty)
                .map(serde_json::to_value)
                .transpose()?
                .map(unwrap_string_value),
            route
                .description
                .as_ref()
                .and_then(|x| x.physical_difficulty)
                .map(serde_json::to_value)
                .transpose()?
                .map(unwrap_string_value),
            route
                .description
                .as_ref()
                .and_then(|x| x.minimum_bike.clone())
                .map(serde_json::to_value)
                .transpose()?,
            route
                .description
                .as_ref()
                .and_then(|x| x.ideal_bike.clone())
                .map(serde_json::to_value)
                .transpose()?,
            route
                .description
                .as_ref()
                .and_then(|x| x.scouted)
                .map(serde_json::to_value)
                .transpose()?
                .map(unwrap_string_value),
            route
                .description
                .as_ref()
                .and_then(|x| x.direction)
                .map(serde_json::to_value)
                .transpose()?
                .map(unwrap_string_value),
            route.description.as_ref().map(|x| &*x.tags).unwrap_or(&[]),
            route.tags.contains(&Tag::BackcountrySegment)
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
