use std::iter;

use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::models::media::{Media, MediaFilter, MediaId, MediaRelationId};
use howitt::models::point_of_interest::PointOfInterestId;
use howitt::models::ride::RideId;
use howitt::models::route::RouteId;
use howitt::models::trip::TripId;
use howitt::models::user::UserId;
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

struct MediaRow {
    id: Uuid,
    created_at: DateTime<Utc>,
    user_id: Uuid,
    path: String,
    ride_ids: Option<Vec<Uuid>>,
    route_ids: Option<Vec<Uuid>>,
    trip_ids: Option<Vec<Uuid>>,
    poi_ids: Option<Vec<Uuid>>,
    point: Option<serde_json::Value>,
    captured_at: Option<DateTime<Utc>>,
}

impl TryFrom<MediaRow> for Media {
    type Error = PostgresRepoError;

    fn try_from(row: MediaRow) -> Result<Self, Self::Error> {
        let relation_ids: Vec<_> = iter::empty()
            .chain(
                row.ride_ids
                    .into_iter()
                    .flatten()
                    .map(RideId::from)
                    .map(MediaRelationId::from),
            )
            .chain(
                row.route_ids
                    .into_iter()
                    .flatten()
                    .map(RouteId::from)
                    .map(MediaRelationId::from),
            )
            .chain(
                row.trip_ids
                    .into_iter()
                    .flatten()
                    .map(TripId::from)
                    .map(MediaRelationId::from),
            )
            .chain(
                row.poi_ids
                    .into_iter()
                    .flatten()
                    .map(PointOfInterestId::from)
                    .map(MediaRelationId::from),
            )
            .collect();

        Ok(Media {
            id: MediaId::from(row.id),
            created_at: row.created_at,
            user_id: UserId::from(row.user_id),
            path: row.path,
            relation_ids,
            point: match row.point {
                Some(point) => Some(serde_json::from_value(point)?),
                None => None,
            },
            captured_at: row.captured_at,
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresMediaRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresMediaRepo {
    type Model = Media;
    type Error = PostgresRepoError;

    async fn filter_models(&self, filter: MediaFilter) -> Result<Vec<Media>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let media = match filter {
            MediaFilter::All => {
                sqlx::query_as!(
                    MediaRow,
                    r#"
                    SELECT
                        m.*,
                        mr.ride_ids,
                        mr.route_ids,
                        mr.trip_ids,
                        mr.poi_ids
                    FROM media m
                    INNER JOIN media_relations mr ON mr.id = m.id
                    ORDER BY created_at DESC
                    "#
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            MediaFilter::Ids(ids) => {
                let uuids: Vec<_> = ids.into_iter().map(Uuid::from).collect();

                sqlx::query_as!(
                    MediaRow,
                    r#"
                    SELECT
                        m.*,
                        mr.ride_ids,
                        mr.route_ids,
                        mr.trip_ids,
                        mr.poi_ids
                    FROM media m
                    INNER JOIN media_relations mr ON mr.id = m.id
                    WHERE m.id = ANY($1)
                    ORDER BY created_at DESC
                    "#,
                    &uuids
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            MediaFilter::ForUser(user_id) => {
                sqlx::query_as!(
                    MediaRow,
                    r#"
                    SELECT
                        m.*,
                        mr.ride_ids,
                        mr.route_ids,
                        mr.trip_ids,
                        mr.poi_ids
                    FROM media m
                    INNER JOIN media_relations mr ON mr.id = m.id
                    WHERE user_id = $1 
                    ORDER BY created_at DESC
                    "#,
                    user_id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            MediaFilter::ForRide(ride_id) => {
                sqlx::query_as!(
                    MediaRow,
                    r#"
                    SELECT
                        m.*,
                        mr.ride_ids,
                        mr.route_ids,
                        mr.trip_ids,
                        mr.poi_ids
                    FROM media m
                    INNER JOIN media_relations mr ON mr.id = m.id
                    INNER JOIN ride_media rm ON rm.media_id = m.id
                    WHERE rm.ride_id = $1
                    ORDER BY m.created_at DESC
                    "#,
                    ride_id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            MediaFilter::ForRoute(route_id) => {
                sqlx::query_as!(
                    MediaRow,
                    r#"
                    SELECT
                        m.*,
                        mr.ride_ids,
                        mr.route_ids,
                        mr.trip_ids,
                        mr.poi_ids
                    FROM media m
                    INNER JOIN media_relations mr ON mr.id = m.id
                    INNER JOIN route_media rm ON rm.media_id = m.id
                    WHERE rm.route_id = $1
                    ORDER BY m.created_at DESC
                    "#,
                    route_id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            MediaFilter::ForTrip(trip_id) => {
                sqlx::query_as!(
                    MediaRow,
                    r#"
                    SELECT
                        m.*,
                        mr.ride_ids,
                        mr.route_ids,
                        mr.trip_ids,
                        mr.poi_ids
                    FROM media m
                    INNER JOIN media_relations mr ON mr.id = m.id
                    INNER JOIN trip_media tm ON tm.media_id = m.id
                    WHERE tm.trip_id = $1
                    ORDER BY m.created_at DESC
                    "#,
                    trip_id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            MediaFilter::ForPointOfInterest(poi_id) => {
                sqlx::query_as!(
                    MediaRow,
                    r#"
                    SELECT
                        m.*,
                        mr.ride_ids,
                        mr.route_ids,
                        mr.trip_ids,
                        mr.poi_ids
                    FROM media m
                    INNER JOIN media_relations mr ON mr.id = m.id
                    INNER JOIN poi_media pm ON pm.media_id = m.id
                    WHERE pm.poi_id = $1
                    ORDER BY m.created_at DESC
                    "#,
                    poi_id.as_uuid()
                )
                .fetch_all(conn.as_mut())
                .await?
            }
        };

        Ok(media
            .into_iter()
            .map(Media::try_from)
            .collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<Media>, PostgresRepoError> {
        self.filter_models(MediaFilter::All).await
    }

    async fn get(&self, id: MediaId) -> Result<Media, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            MediaRow,
            r#"
            SELECT
                m.*,
                mr.ride_ids,
                mr.route_ids,
                mr.trip_ids,
                mr.poi_ids
            FROM media m
            INNER JOIN media_relations mr ON mr.id = m.id
            WHERE m.id = $1
            "#,
            id.as_uuid()
        );

        Ok(Media::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }

    async fn put(&self, media: Media) -> Result<(), PostgresRepoError> {
        let mut tx = self.client.begin().await?;

        // Insert/update the media record
        let query = sqlx::query!(
            r#"
            INSERT INTO media (
                id,
                created_at,
                user_id,
                path,
                point,
                captured_at
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE 
            SET path = EXCLUDED.path,
                point = EXCLUDED.point,
                captured_at = EXCLUDED.captured_at
            "#,
            media.id.as_uuid(),
            media.created_at,
            media.user_id.as_uuid(),
            media.path,
            media.point.map(|p| serde_json::to_value(p).unwrap()),
            media.captured_at,
        );
        query.execute(tx.as_mut()).await?;

        // Handle ride relations
        let ride_ids: Vec<_> = media.iter_ride_ids().map(|id| *id.as_uuid()).collect();

        sqlx::query!(
            r#"
            DELETE FROM ride_media 
            WHERE media_id = $1 
            AND ride_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
            "#,
            media.id.as_uuid(),
            &ride_ids,
        )
        .execute(tx.as_mut())
        .await?;

        for ride_id in ride_ids {
            sqlx::query!(
                r#"
                INSERT INTO ride_media (ride_id, media_id) 
                VALUES ($1, $2)
                ON CONFLICT (ride_id, media_id) DO NOTHING
                "#,
                ride_id,
                media.id.as_uuid(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        // Handle route relations
        let route_ids: Vec<_> = media.iter_route_ids().map(|id| *id.as_uuid()).collect();
        sqlx::query!(
            r#"
            DELETE FROM route_media 
            WHERE media_id = $1 
            AND route_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
            "#,
            media.id.as_uuid(),
            &route_ids,
        )
        .execute(tx.as_mut())
        .await?;

        for route_id in route_ids {
            sqlx::query!(
                r#"
                INSERT INTO route_media (route_id, media_id) 
                VALUES ($1, $2)
                ON CONFLICT (route_id, media_id) DO NOTHING
                "#,
                route_id,
                media.id.as_uuid(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        // Handle trip relations
        let trip_ids: Vec<_> = media.iter_trip_ids().map(|id| *id.as_uuid()).collect();

        sqlx::query!(
            r#"
            DELETE FROM trip_media 
            WHERE media_id = $1 
            AND trip_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
            "#,
            media.id.as_uuid(),
            &trip_ids,
        )
        .execute(tx.as_mut())
        .await?;

        for trip_id in trip_ids {
            sqlx::query!(
                r#"
                INSERT INTO trip_media (trip_id, media_id) 
                VALUES ($1, $2)
                ON CONFLICT (trip_id, media_id) DO NOTHING
                "#,
                trip_id,
                media.id.as_uuid(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        // Handle point of interest relations
        let poi_ids: Vec<_> = media
            .iter_point_of_interest_ids()
            .map(|id| *id.as_uuid())
            .collect();

        sqlx::query!(
            r#"
            DELETE FROM poi_media 
            WHERE media_id = $1 
            AND poi_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
            "#,
            media.id.as_uuid(),
            &poi_ids,
        )
        .execute(tx.as_mut())
        .await?;

        for poi_id in poi_ids {
            sqlx::query!(
                r#"
                INSERT INTO poi_media (poi_id, media_id) 
                VALUES ($1, $2)
                ON CONFLICT (poi_id, media_id) DO NOTHING
                "#,
                poi_id,
                media.id.as_uuid(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
