use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::ext::serde::json::unwrap_string_value;
use howitt::models::route::{Route, RouteFilter, RouteId};
use howitt::models::route_description::RouteDescription;
use howitt::models::tag::Tag;
use howitt::models::user::UserId;

use howitt::models::Model;
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

#[allow(dead_code)]
struct RouteIndexRow {
    id: Uuid,
    created_at: DateTime<Utc>,
    name: String,
    slug: String,
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
            id: RouteId::from(row.id),
            name: row.name,
            slug: row.slug,
            user_id: UserId::from(row.user_id),
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
                .chain(if row.is_starred {
                    vec![Tag::Starred]
                } else {
                    vec![]
                })
                .collect(),
        })
    }
}

#[allow(dead_code)]
struct RouteRow {
    id: Uuid,
    created_at: DateTime<Utc>,
    name: String,
    slug: String,
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

impl TryFrom<RouteRow> for Route {
    type Error = PostgresRepoError;

    fn try_from(row: RouteRow) -> Result<Self, Self::Error> {
        Ok(Route {
            id: RouteId::from(row.id),
            name: row.name,
            slug: row.slug,
            user_id: UserId::from(row.user_id),
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
                .chain(if row.is_starred {
                    vec![Tag::Starred]
                } else {
                    vec![]
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresRouteRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresRouteRepo {
    type Model = Route;
    type Error = PostgresRepoError;

    async fn filter_models(&self, filter: RouteFilter) -> Result<Vec<Route>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let rows = match filter {
            RouteFilter::Starred => {
                sqlx::query_as!(RouteRow, r#"select * from routes where is_starred = true"#)
                    .fetch_all(conn.as_mut())
                    .await?
            }
            RouteFilter::All => {
                sqlx::query_as!(RouteRow, r#"select * from routes"#)
                    .fetch_all(conn.as_mut())
                    .await?
            }
            RouteFilter::Slug(slug) => {
                sqlx::query_as!(RouteRow, r#"select * from routes where slug = $1"#, slug)
                    .fetch_all(conn.as_mut())
                    .await?
            }
            RouteFilter::RwgpsId(rwgps_id) => sqlx::query_as!(
                RouteRow,
                r#"select * from routes where (external_ref->'id'->'Rwgps'->'Route')::int = $1"#,
                rwgps_id as i32
            )
            .fetch_all(conn.as_mut())
            .await?,
            RouteFilter::UserId(user_id) => {
                sqlx::query_as!(
                    RouteRow,
                    r#"select * from routes where user_id = $1"#,
                    user_id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await?
            }
        };

        Ok(rows.into_iter().map(Route::try_from).collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<Route>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RouteIndexRow,
            r#"select id,
                created_at,
                name,
                slug,
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
    async fn get(&self, id: <Route as Model>::Id) -> Result<Route, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            RouteRow,
            r#"select * from routes where id = $1"#,
            id.as_uuid()
        );

        Ok(Route::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }

    async fn put(&self, route: Route) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query!(
            r#"insert into routes (
                id,
                created_at,
                name,
                slug,
                external_ref,
                sample_points,
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
                is_starred,
                user_id
            ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                slug = EXCLUDED.slug,
                external_ref = EXCLUDED.external_ref,
                sample_points = EXCLUDED.sample_points,
                distance_m = EXCLUDED.distance_m,
                description = EXCLUDED.description,
                published_at = EXCLUDED.published_at,
                technical_difficulty = EXCLUDED.technical_difficulty,
                physical_difficulty = EXCLUDED.physical_difficulty,
                minimum_bike = EXCLUDED.minimum_bike,
                ideal_bike = EXCLUDED.ideal_bike,
                scouted = EXCLUDED.scouted,
                direction = EXCLUDED.direction,
                tags = EXCLUDED.tags,
                is_starred = EXCLUDED.is_starred"#,
            route.id.as_uuid(),
            Utc::now(),
            route.name,
            route.slug,
            route.external_ref.map(serde_json::to_value).transpose()?,
            route.sample_points.map(serde_json::to_value).transpose()?,
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
            route.tags.contains(&Tag::BackcountrySegment),
            route.user_id.as_uuid()
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
