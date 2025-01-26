use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::models::media::{Media, MediaFilter, MediaId, MediaRelationId};
use howitt::models::point_of_interest::PointOfInterestId;
use howitt::models::ride::RideId;
use howitt::models::route::RouteId;
use howitt::models::trip::TripId;
use howitt::models::user::UserId;
use howitt::models::Model;
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
}

impl TryFrom<MediaRow> for Media {
    type Error = PostgresRepoError;

    fn try_from(row: MediaRow) -> Result<Self, Self::Error> {
        let mut relation_ids = Vec::new();

        // Add ride relations
        if let Some(ride_ids) = row.ride_ids {
            relation_ids.extend(
                ride_ids
                    .into_iter()
                    .map(|id| MediaRelationId::Ride(RideId::from(id))),
            );
        }

        // Add route relations
        if let Some(route_ids) = row.route_ids {
            relation_ids.extend(
                route_ids
                    .into_iter()
                    .map(|id| MediaRelationId::Route(RouteId::from(id))),
            );
        }

        // Add trip relations
        if let Some(trip_ids) = row.trip_ids {
            relation_ids.extend(
                trip_ids
                    .into_iter()
                    .map(|id| MediaRelationId::Trip(TripId::from(id))),
            );
        }

        // Add POI relations
        if let Some(poi_ids) = row.poi_ids {
            relation_ids.extend(
                poi_ids
                    .into_iter()
                    .map(|id| MediaRelationId::PointOfInterest(PointOfInterestId::from(id))),
            );
        }

        Ok(Media {
            id: MediaId::from(row.id),
            created_at: row.created_at,
            user_id: UserId::from(row.user_id),
            path: row.path,
            relation_ids,
        })
    }
}

#[derive(Debug, derive_more::Constructor)]
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

    async fn all_indexes(&self) -> Result<Vec<Media>, PostgresRepoError> {
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

    async fn get_index(
        &self,
        id: MediaId,
    ) -> Result<<Media as Model>::IndexItem, PostgresRepoError> {
        self.get(id).await
    }

    async fn put(&self, media: Media) -> Result<(), PostgresRepoError> {
        let mut tx = self.client.begin().await?;

        let query = sqlx::query!(
            r#"
            INSERT INTO media (
                id,
                created_at,
                user_id,
                path
            ) VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE 
            SET path = EXCLUDED.path
            "#,
            media.id.as_uuid(),
            media.created_at,
            media.user_id.as_uuid(),
            media.path,
        );

        query.execute(tx.as_mut()).await?;
        tx.commit().await?;

        Ok(())
    }
}
