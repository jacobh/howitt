use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;
use howitt::models::media::{Media, MediaFilter, MediaId};
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
}

impl TryFrom<MediaRow> for Media {
    type Error = PostgresRepoError;

    fn try_from(row: MediaRow) -> Result<Self, Self::Error> {
        Ok(Media {
            id: MediaId::from(row.id),
            created_at: row.created_at,
            user_id: UserId::from(row.user_id),
            path: row.path,
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
                sqlx::query_as!(MediaRow, r#"SELECT * FROM media ORDER BY created_at DESC"#)
                    .fetch_all(conn.as_mut())
                    .await?
            }
            MediaFilter::ForUser(user_id) => {
                sqlx::query_as!(
                    MediaRow,
                    r#"
                    SELECT * FROM media 
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
                    SELECT m.* 
                    FROM media m
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
                    SELECT m.* 
                    FROM media m
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
                    SELECT m.* 
                    FROM media m
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
                    SELECT m.* 
                    FROM media m
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
            r#"SELECT * FROM media WHERE id = $1"#,
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
