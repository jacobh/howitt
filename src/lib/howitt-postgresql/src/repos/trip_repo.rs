use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::models::trip::{Trip, TripFilter, TripId};
use howitt::models::user::UserId;
use howitt::models::{media::MediaId, ride::RideId};
use howitt::repos::Repo;
use itertools::Itertools;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

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
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresTripRepo {
    type Model = Trip;
    type Error = PostgresRepoError;

    async fn filter_models(&self, filter: TripFilter) -> Result<Vec<Trip>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let trips = match filter {
            TripFilter::User(user_id) => {
                sqlx::query_as!(
                    TripRow,
                    r#"
                        SELECT 
                            t.*,
                            tr.ride_ids,
                            tr.media_ids
                        FROM trips t
                        INNER JOIN trip_relations tr ON tr.id = t.id
                        WHERE user_id = $1
                    "#,
                    user_id.as_uuid(),
                )
                .fetch_all(conn.as_mut())
                .await
            }
            TripFilter::WithUserAndSlug { user_id, slug } => {
                sqlx::query_as!(
                    TripRow,
                    r#"
                        SELECT 
                            t.*,
                            tr.ride_ids,
                            tr.media_ids
                        FROM trips t
                        INNER JOIN trip_relations tr ON tr.id = t.id
                        WHERE user_id = $1 AND slug = $2
                    "#,
                    user_id.as_uuid(),
                    slug,
                )
                .fetch_all(conn.as_mut())
                .await
            }
            TripFilter::All => {
                sqlx::query_as!(
                    TripRow,
                    r#"
                        SELECT 
                            t.*,
                            tr.ride_ids,
                            tr.media_ids
                        FROM trips t
                        INNER JOIN trip_relations tr ON tr.id = t.id
                    "#
                )
                .fetch_all(conn.as_mut())
                .await
            }
            TripFilter::Published => {
                sqlx::query_as!(
                    TripRow,
                    r#"
                        SELECT 
                            t.*,
                            tr.ride_ids,
                            tr.media_ids
                        FROM trips t
                        INNER JOIN trip_relations tr ON tr.id = t.id
                        WHERE t.is_published = TRUE
                    "#
                )
                .fetch_all(conn.as_mut())
                .await
            }
        }?;

        Ok(trips.into_iter().map(Trip::try_from).collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<Trip>, PostgresRepoError> {
        self.filter_models(TripFilter::All).await
    }

    async fn get(&self, id: TripId) -> Result<Trip, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            TripRow,
            r#"
                SELECT 
                    t.*,
                    tr.ride_ids,
                    tr.media_ids
                FROM trips t
                INNER JOIN trip_relations tr ON tr.id = t.id
                WHERE t.id = $1
            "#,
            id.as_uuid()
        );

        Ok(Trip::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }

    async fn put(&self, trip: Trip) -> Result<(), PostgresRepoError> {
        let mut tx = self.client.begin().await?;

        let query = sqlx::query!(
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
            trip.id.as_uuid(),
            trip.name,
            trip.slug,
            trip.year,
            trip.description,
            trip.created_at,
            trip.user_id.as_uuid(),
            serde_json::to_value(&trip.notes)?,
            trip.is_published,
        );

        query.execute(tx.as_mut()).await?;

        // Update ride associations
        sqlx::query!(
            r#"
            DELETE FROM trip_rides 
            WHERE trip_id = $1 
            AND ride_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
        "#,
            trip.id.as_uuid(),
            &trip.ride_ids.iter().map(|id| *id.as_uuid()).collect_vec(),
        )
        .execute(tx.as_mut())
        .await?;

        for ride_id in trip.ride_ids {
            let query = sqlx::query!(
                r#"
                INSERT INTO trip_rides (
                    trip_id,
                    ride_id
                ) VALUES ($1, $2)
                ON CONFLICT (trip_id, ride_id) DO NOTHING
            "#,
                *trip.id.as_uuid(),
                *ride_id.as_uuid(),
            );

            query.execute(tx.as_mut()).await?;
        }

        // Update media associations
        sqlx::query!(
            r#"
            DELETE FROM trip_media 
            WHERE trip_id = $1 
            AND media_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
        "#,
            trip.id.as_uuid(),
            &trip.media_ids.iter().map(|id| *id.as_uuid()).collect_vec(),
        )
        .execute(tx.as_mut())
        .await?;

        for media_id in trip.media_ids {
            let query = sqlx::query!(
                r#"
                INSERT INTO trip_media (
                    trip_id,
                    media_id
                ) VALUES ($1, $2)
                ON CONFLICT (trip_id, media_id) DO NOTHING
            "#,
                *trip.id.as_uuid(),
                *media_id.as_uuid(),
            );

            query.execute(tx.as_mut()).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
