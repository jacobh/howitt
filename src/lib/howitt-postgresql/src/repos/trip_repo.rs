use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::models::trip::{Trip, TripFilter, TripId};
use howitt::models::user::UserId;
use howitt::models::{media::MediaId, ride::RideId};
use howitt::repos::Repo;
use itertools::Itertools;
use tokio_postgres::types::Type;
use uuid::Uuid;

use crate::{PostgresPool, PostgresRepoError};

struct TripRow {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    slug: String,
    year: i32,
    description: Option<String>,
    user_id: Uuid,
    notes: Option<serde_json::Value>,
    ride_ids: Option<Vec<Uuid>>,
    media_ids: Option<Vec<Uuid>>,
    is_published: bool,
}

impl TryFrom<&tokio_postgres::Row> for TripRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            created_at: row.try_get("created_at")?,
            slug: row.try_get("slug")?,
            year: row.try_get("year")?,
            description: row.try_get("description")?,
            user_id: row.try_get("user_id")?,
            notes: row.try_get("notes")?,
            ride_ids: row.try_get("ride_ids")?,
            media_ids: row.try_get("media_ids")?,
            is_published: row.try_get("is_published")?,
        })
    }
}

impl TryFrom<TripRow> for Trip {
    type Error = PostgresRepoError;

    fn try_from(row: TripRow) -> Result<Self, Self::Error> {
        Ok(Trip {
            id: TripId::from(row.id),
            name: row.name,
            slug: row.slug,
            year: row.year,
            description: row.description,
            created_at: row.created_at,
            user_id: UserId::from(row.user_id),
            notes: row
                .notes
                .map(|n| serde_json::from_value(n))
                .transpose()?
                .unwrap_or_default(),
            ride_ids: row
                .ride_ids
                .unwrap_or_default()
                .into_iter()
                .map(RideId::from)
                .collect(),
            media_ids: row
                .media_ids
                .unwrap_or_default()
                .into_iter()
                .map(MediaId::from)
                .collect(),
            is_published: row.is_published,
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresTripRepo {
    pool: PostgresPool,
}

#[async_trait::async_trait]
impl Repo for PostgresTripRepo {
    type Model = Trip;
    type Error = PostgresRepoError;

    async fn filter_models(&self, filter: TripFilter) -> Result<Vec<Trip>, PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        let trips = match filter {
            TripFilter::User(user_id) => conn
                .query_typed(
                    r#"
                        SELECT
                            t.*,
                            tr.ride_ids,
                            tr.media_ids
                        FROM trips t
                        INNER JOIN trip_relations tr ON tr.id = t.id
                        WHERE user_id = $1
                    "#,
                    &[(&(user_id.as_uuid()), Type::UUID)],
                )
                .await?
                .iter()
                .map(TripRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            TripFilter::WithUserAndSlug { user_id, slug } => conn
                .query_typed(
                    r#"
                        SELECT
                            t.*,
                            tr.ride_ids,
                            tr.media_ids
                        FROM trips t
                        INNER JOIN trip_relations tr ON tr.id = t.id
                        WHERE user_id = $1 AND slug = $2
                    "#,
                    &[(&(user_id.as_uuid()), Type::UUID), (&(slug), Type::VARCHAR)],
                )
                .await?
                .iter()
                .map(TripRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            TripFilter::All => conn
                .query_typed(
                    r#"
                        SELECT
                            t.*,
                            tr.ride_ids,
                            tr.media_ids
                        FROM trips t
                        INNER JOIN trip_relations tr ON tr.id = t.id
                    "#,
                    &[],
                )
                .await?
                .iter()
                .map(TripRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            TripFilter::Published => conn
                .query_typed(
                    r#"
                        SELECT
                            t.*,
                            tr.ride_ids,
                            tr.media_ids
                        FROM trips t
                        INNER JOIN trip_relations tr ON tr.id = t.id
                        WHERE t.is_published = TRUE
                    "#,
                    &[],
                )
                .await?
                .iter()
                .map(TripRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        };

        Ok(trips.into_iter().map(Trip::try_from).collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<Trip>, PostgresRepoError> {
        self.filter_models(TripFilter::All).await
    }

    async fn get(&self, id: TripId) -> Result<Trip, PostgresRepoError> {
        let conn = self.pool.acquire().await?;

        Ok(Trip::try_from(TripRow::try_from(
            &conn
                .query_typed_one(
                    r#"
                SELECT
                    t.*,
                    tr.ride_ids,
                    tr.media_ids
                FROM trips t
                INNER JOIN trip_relations tr ON tr.id = t.id
                WHERE t.id = $1
            "#,
                    &[(&(id.as_uuid()), Type::UUID)],
                )
                .await?,
        )?)?)
    }

    async fn put(&self, trip: Trip) -> Result<(), PostgresRepoError> {
        let mut conn = self.pool.acquire().await?;
        let tx = conn.transaction().await?;

        tx.execute_typed(
            r#"
                INSERT INTO trips (
                    id,
                    name,
                    slug,
                    year,
                    description,
                    created_at,
                    user_id,
                    notes,
                    is_published
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT (id) DO UPDATE
                SET
                    name = EXCLUDED.name,
                    slug = EXCLUDED.slug,
                    year = EXCLUDED.year,
                    description = EXCLUDED.description,
                    notes = EXCLUDED.notes,
                    is_published = EXCLUDED.is_published
            "#,
            &[
                (trip.id.as_uuid(), Type::UUID),
                (&trip.name, Type::VARCHAR),
                (&trip.slug, Type::VARCHAR),
                (&trip.year, Type::INT4),
                (&trip.description, Type::TEXT),
                (&trip.created_at, Type::TIMESTAMPTZ),
                (trip.user_id.as_uuid(), Type::UUID),
                (&serde_json::to_value(&trip.notes)?, Type::JSONB),
                (&trip.is_published, Type::BOOL),
            ],
        )
        .await?;

        // Update ride associations
        tx.execute_typed(
            r#"
            DELETE FROM trip_rides
            WHERE trip_id = $1
            AND ride_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
        "#,
            &[
                (trip.id.as_uuid(), Type::UUID),
                (
                    &trip.ride_ids.iter().map(|id| *id.as_uuid()).collect_vec(),
                    Type::UUID_ARRAY,
                ),
            ],
        )
        .await?;

        for ride_id in trip.ride_ids {
            tx.execute_typed(
                r#"
                INSERT INTO trip_rides (
                    trip_id,
                    ride_id
                ) VALUES ($1, $2)
                ON CONFLICT (trip_id, ride_id) DO NOTHING
            "#,
                &[
                    (trip.id.as_uuid(), Type::UUID),
                    (ride_id.as_uuid(), Type::UUID),
                ],
            )
            .await?;
        }

        // Update media associations
        tx.execute_typed(
            r#"
            DELETE FROM trip_media
            WHERE trip_id = $1
            AND media_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
        "#,
            &[
                (trip.id.as_uuid(), Type::UUID),
                (
                    &trip.media_ids.iter().map(|id| *id.as_uuid()).collect_vec(),
                    Type::UUID_ARRAY,
                ),
            ],
        )
        .await?;

        for media_id in trip.media_ids {
            tx.execute_typed(
                r#"
                INSERT INTO trip_media (
                    trip_id,
                    media_id
                ) VALUES ($1, $2)
                ON CONFLICT (trip_id, media_id) DO NOTHING
            "#,
                &[
                    (trip.id.as_uuid(), Type::UUID),
                    (media_id.as_uuid(), Type::UUID),
                ],
            )
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
