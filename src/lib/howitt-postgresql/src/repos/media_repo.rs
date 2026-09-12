use std::iter;
use tokio_postgres::types::Type;

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

impl TryFrom<&tokio_postgres::Row> for MediaRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            user_id: row.try_get("user_id")?,
            path: row.try_get("path")?,
            ride_ids: row.try_get("ride_ids")?,
            route_ids: row.try_get("route_ids")?,
            trip_ids: row.try_get("trip_ids")?,
            poi_ids: row.try_get("poi_ids")?,
            point: row.try_get("point")?,
            captured_at: row.try_get("captured_at")?,
        })
    }
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
        let conn = self.client.acquire().await?;

        let media = match filter {
            MediaFilter::All => conn
                .query_typed(
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
                    "#,
                    &[],
                )
                .await?
                .iter()
                .map(MediaRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            MediaFilter::Ids(ids) => {
                let uuids: Vec<_> = ids.into_iter().map(Uuid::from).collect();

                conn.query_typed(
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
                    &[(&uuids, Type::UUID_ARRAY)],
                )
                .await?
                .iter()
                .map(MediaRow::try_from)
                .collect::<Result<Vec<_>, _>>()?
            }
            MediaFilter::ForUser(user_id) => conn
                .query_typed(
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
                    &[(&(user_id.as_uuid()), Type::UUID)],
                )
                .await?
                .iter()
                .map(MediaRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            MediaFilter::ForRide(ride_id) => conn
                .query_typed(
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
                    &[(&(ride_id.as_uuid()), Type::UUID)],
                )
                .await?
                .iter()
                .map(MediaRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            MediaFilter::ForRoute(route_id) => conn
                .query_typed(
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
                    &[(&(route_id.as_uuid()), Type::UUID)],
                )
                .await?
                .iter()
                .map(MediaRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            MediaFilter::ForTrip(trip_id) => conn
                .query_typed(
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
                    &[(&(trip_id.as_uuid()), Type::UUID)],
                )
                .await?
                .iter()
                .map(MediaRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            MediaFilter::ForPointOfInterest(poi_id) => conn
                .query_typed(
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
                    &[(&(poi_id.as_uuid()), Type::UUID)],
                )
                .await?
                .iter()
                .map(MediaRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
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
        let conn = self.client.acquire().await?;

        Ok(Media::try_from(MediaRow::try_from(
            &conn
                .query_typed_one(
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
                    &[(&(id.as_uuid()), Type::UUID)],
                )
                .await?,
        )?)?)
    }

    async fn put(&self, media: Media) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await?;
        let tx = conn.transaction().await?;

        // Insert/update the media record

        tx.execute_typed(
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
            &[
                (media.id.as_uuid(), Type::UUID),
                (&media.created_at, Type::TIMESTAMPTZ),
                (media.user_id.as_uuid(), Type::UUID),
                (&media.path, Type::VARCHAR),
                (
                    &media.point.map(|p| serde_json::to_value(p).unwrap()),
                    Type::JSONB,
                ),
                (&media.captured_at, Type::TIMESTAMPTZ),
            ],
        )
        .await?;

        // Handle ride relations
        let ride_ids: Vec<_> = media.iter_ride_ids().map(|id| *id.as_uuid()).collect();

        tx.execute_typed(
            r#"
            DELETE FROM ride_media
            WHERE media_id = $1
            AND ride_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
            "#,
            &[
                (media.id.as_uuid(), Type::UUID),
                (&ride_ids, Type::UUID_ARRAY),
            ],
        )
        .await?;

        for ride_id in ride_ids {
            tx.execute_typed(
                r#"
                INSERT INTO ride_media (ride_id, media_id)
                VALUES ($1, $2)
                ON CONFLICT (ride_id, media_id) DO NOTHING
                "#,
                &[(&ride_id, Type::UUID), (media.id.as_uuid(), Type::UUID)],
            )
            .await?;
        }

        // Handle route relations
        let route_ids: Vec<_> = media.iter_route_ids().map(|id| *id.as_uuid()).collect();
        tx.execute_typed(
            r#"
            DELETE FROM route_media
            WHERE media_id = $1
            AND route_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
            "#,
            &[
                (media.id.as_uuid(), Type::UUID),
                (&route_ids, Type::UUID_ARRAY),
            ],
        )
        .await?;

        for route_id in route_ids {
            tx.execute_typed(
                r#"
                INSERT INTO route_media (route_id, media_id)
                VALUES ($1, $2)
                ON CONFLICT (route_id, media_id) DO NOTHING
                "#,
                &[(&route_id, Type::UUID), (media.id.as_uuid(), Type::UUID)],
            )
            .await?;
        }

        // Handle trip relations
        let trip_ids: Vec<_> = media.iter_trip_ids().map(|id| *id.as_uuid()).collect();

        tx.execute_typed(
            r#"
            DELETE FROM trip_media
            WHERE media_id = $1
            AND trip_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
            "#,
            &[
                (media.id.as_uuid(), Type::UUID),
                (&trip_ids, Type::UUID_ARRAY),
            ],
        )
        .await?;

        for trip_id in trip_ids {
            tx.execute_typed(
                r#"
                INSERT INTO trip_media (trip_id, media_id)
                VALUES ($1, $2)
                ON CONFLICT (trip_id, media_id) DO NOTHING
                "#,
                &[(&trip_id, Type::UUID), (media.id.as_uuid(), Type::UUID)],
            )
            .await?;
        }

        // Handle point of interest relations
        let poi_ids: Vec<_> = media
            .iter_point_of_interest_ids()
            .map(|id| *id.as_uuid())
            .collect();

        tx.execute_typed(
            r#"
            DELETE FROM poi_media
            WHERE media_id = $1
            AND poi_id NOT IN (SELECT * FROM UNNEST($2::uuid[]))
            "#,
            &[
                (media.id.as_uuid(), Type::UUID),
                (&poi_ids, Type::UUID_ARRAY),
            ],
        )
        .await?;

        for poi_id in poi_ids {
            tx.execute_typed(
                r#"
                INSERT INTO poi_media (poi_id, media_id)
                VALUES ($1, $2)
                ON CONFLICT (poi_id, media_id) DO NOTHING
                "#,
                &[(&poi_id, Type::UUID), (media.id.as_uuid(), Type::UUID)],
            )
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
