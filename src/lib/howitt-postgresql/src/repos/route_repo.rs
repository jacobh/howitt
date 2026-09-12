use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::ext::serde::json::unwrap_string_value;
use howitt::models::route::{Route, RouteFilter, RouteId};
use howitt::models::route_description::RouteDescription;
use howitt::models::tag::Tag;
use howitt::models::user::UserId;
use tokio_postgres::types::Type;

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

impl TryFrom<&tokio_postgres::Row> for RouteIndexRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            external_ref: row.try_get("external_ref")?,
            distance_m: row.try_get("distance_m")?,
            sample_points: row.try_get("sample_points")?,
            description: row.try_get("description")?,
            published_at: row.try_get("published_at")?,
            technical_difficulty: row.try_get("technical_difficulty")?,
            physical_difficulty: row.try_get("physical_difficulty")?,
            minimum_bike: row.try_get("minimum_bike")?,
            ideal_bike: row.try_get("ideal_bike")?,
            scouted: row.try_get("scouted")?,
            direction: row.try_get("direction")?,
            tags: row.try_get("tags")?,
            is_starred: row.try_get("is_starred")?,
            user_id: row.try_get("user_id")?,
        })
    }
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

impl TryFrom<&tokio_postgres::Row> for RouteRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            external_ref: row.try_get("external_ref")?,
            distance_m: row.try_get("distance_m")?,
            sample_points: row.try_get("sample_points")?,
            description: row.try_get("description")?,
            published_at: row.try_get("published_at")?,
            technical_difficulty: row.try_get("technical_difficulty")?,
            physical_difficulty: row.try_get("physical_difficulty")?,
            minimum_bike: row.try_get("minimum_bike")?,
            ideal_bike: row.try_get("ideal_bike")?,
            scouted: row.try_get("scouted")?,
            direction: row.try_get("direction")?,
            tags: row.try_get("tags")?,
            is_starred: row.try_get("is_starred")?,
            user_id: row.try_get("user_id")?,
        })
    }
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
        let conn = self.client.acquire().await?;

        let rows = match filter {
            RouteFilter::Starred => {
                conn.query_typed(
                    r#"select * from routes where is_starred = true"#,
                    &[],
                ).await?.iter().map(RouteRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RouteFilter::All => {
                conn.query_typed(
                    r#"select * from routes"#,
                    &[],
                ).await?.iter().map(RouteRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RouteFilter::Slug(slug) => {
                conn.query_typed(
                    r#"select * from routes where slug = $1"#,
                    &[(&slug, Type::VARCHAR)],
                ).await?.iter().map(RouteRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
            RouteFilter::RwgpsId(rwgps_id) => conn.query_typed(
                r#"select * from routes where (external_ref->'id'->'Rwgps'->'Route')::int = $1"#,
                &[(&(rwgps_id as i32), Type::INT4)],
            ).await?.iter().map(RouteRow::try_from).collect::<Result<Vec<_>, _>>()?,
            RouteFilter::UserId(user_id) => {
                conn.query_typed(
                    r#"select * from routes where user_id = $1"#,
                    &[(user_id.as_uuid(), Type::UUID)],
                ).await?.iter().map(RouteRow::try_from).collect::<Result<Vec<_>, _>>()?
            }
        };

        Ok(rows.into_iter().map(Route::try_from).collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<Route>, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        Ok(conn
            .query_typed(
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
            from routes"#,
                &[],
            )
            .await?
            .iter()
            .map(RouteIndexRow::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(Route::try_from)
            .collect_result_vec()?)
    }
    async fn get(&self, id: <Route as Model>::Id) -> Result<Route, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        Ok(Route::try_from(RouteRow::try_from(
            &conn
                .query_typed_one(
                    r#"select * from routes where id = $1"#,
                    &[(&(id.as_uuid()), Type::UUID)],
                )
                .await?,
        )?)?)
    }

    async fn put(&self, route: Route) -> Result<(), PostgresRepoError> {
        let conn = self.client.acquire().await?;

        conn.execute_typed(
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
            &[(route.id.as_uuid(), Type::UUID), (&Utc::now(), Type::TIMESTAMPTZ), (&route.name, Type::TEXT), (&route.slug, Type::VARCHAR), (&route.external_ref.map(serde_json::to_value).transpose()?, Type::JSONB), (&route.sample_points.map(serde_json::to_value).transpose()?, Type::JSONB), (&(route.distance as i32), Type::INT4), (&route
                    .description
                    .as_ref()
                    .and_then(|x| x.description.clone()), Type::TEXT), (&route
                    .description
                    .as_ref()
                    .and_then(|x| x.published_at.clone()), Type::TIMESTAMPTZ), (&route
                    .description
                    .as_ref()
                    .and_then(|x| x.technical_difficulty)
                    .map(serde_json::to_value)
                    .transpose()?
                    .map(unwrap_string_value), Type::VARCHAR), (&route
                    .description
                    .as_ref()
                    .and_then(|x| x.physical_difficulty)
                    .map(serde_json::to_value)
                    .transpose()?
                    .map(unwrap_string_value), Type::VARCHAR), (&route
                    .description
                    .as_ref()
                    .and_then(|x| x.minimum_bike.clone())
                    .map(serde_json::to_value)
                    .transpose()?, Type::JSONB), (&route
                    .description
                    .as_ref()
                    .and_then(|x| x.ideal_bike.clone())
                    .map(serde_json::to_value)
                    .transpose()?, Type::JSONB), (&route
                    .description
                    .as_ref()
                    .and_then(|x| x.scouted)
                    .map(serde_json::to_value)
                    .transpose()?
                    .map(unwrap_string_value), Type::VARCHAR), (&route
                    .description
                    .as_ref()
                    .and_then(|x| x.direction)
                    .map(serde_json::to_value)
                    .transpose()?
                    .map(unwrap_string_value), Type::VARCHAR), (&route.description.as_ref().map(|x| &*x.tags).unwrap_or(&[]), Type::VARCHAR_ARRAY), (&route.tags.contains(&Tag::BackcountrySegment), Type::BOOL), (route.user_id.as_uuid(), Type::UUID)],
        ).await?;

        Ok(())
    }
}
