use argon2::password_hash::Encoding;
use argon2::PasswordHash;
use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;

use howitt::models::user::{UserFilter, UserId, UserRwgpsConnection};
use howitt::models::{user::User, Model};
use howitt::repos::Repo;
use uuid::Uuid;

use crate::{PostgresClient, PostgresRepoError};

struct UserRow {
    id: Uuid,
    username: String,
    password: String,
    email: String,
    created_at: DateTime<Utc>,

    // rwgps connection fields
    rwgps_id: Option<Uuid>,
    rwgps_user_id: Option<i32>,
    rwgps_access_token: Option<String>,
    rwgps_created_at: Option<DateTime<Utc>>,
    rwgps_updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<UserRow> for User {
    type Error = PostgresRepoError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let rwgps_connection = match (row.rwgps_id, row.rwgps_user_id, row.rwgps_access_token) {
            (Some(id), Some(user_id), Some(access_token)) => Some(UserRwgpsConnection {
                id,
                user_id: UserId::from(row.id),
                rwgps_user_id: user_id,
                access_token,
                created_at: row.rwgps_created_at.unwrap(),
                updated_at: row.rwgps_updated_at.unwrap(),
            }),
            _ => None,
        };

        Ok(User {
            id: UserId::from(row.id),
            username: row.username,
            password: PasswordHash::parse(&row.password, Encoding::default())
                .unwrap()
                .serialize(),
            email: row.email,
            created_at: row.created_at,
            rwgps_connection,
        })
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct PostgresUserRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresUserRepo {
    type Model = User;
    type Error = PostgresRepoError;

    async fn filter_models(&self, filter: UserFilter) -> Result<Vec<User>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let users = match filter {
            UserFilter::Ids(ids) => {
                let uuids: Vec<_> = ids.into_iter().map(Uuid::from).collect();

                sqlx::query_as!(
                    UserRow,
                    r#"
                    SELECT 
                        u.*,
                        rc.id as "rwgps_id?",
                        rc.rwgps_user_id as "rwgps_user_id?",
                        rc.access_token as "rwgps_access_token?",
                        rc.created_at as "rwgps_created_at?",
                        rc.updated_at as "rwgps_updated_at?"
                    FROM users u
                    LEFT JOIN user_rwgps_connections rc ON rc.user_id = u.id
                    WHERE u.id = ANY($1)
                    "#,
                    &uuids
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            UserFilter::Username(username) => {
                sqlx::query_as!(
                    UserRow,
                    r#"
                    SELECT 
                        u.*,
                        rc.id as "rwgps_id?",
                        rc.rwgps_user_id as "rwgps_user_id?",
                        rc.access_token as "rwgps_access_token?",
                        rc.created_at as "rwgps_created_at?",
                        rc.updated_at as "rwgps_updated_at?"
                    FROM users u
                    LEFT JOIN user_rwgps_connections rc ON rc.user_id = u.id
                    WHERE u.username = $1
                    "#,
                    username
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            UserFilter::RwgpsId(rwgps_user_id) => {
                sqlx::query_as!(
                    UserRow,
                    r#"
                SELECT 
                    u.*,
                    rc.id as "rwgps_id?",
                    rc.rwgps_user_id as "rwgps_user_id?",
                    rc.access_token as "rwgps_access_token?",
                    rc.created_at as "rwgps_created_at?",
                    rc.updated_at as "rwgps_updated_at?"
                FROM users u
                INNER JOIN user_rwgps_connections rc ON rc.user_id = u.id
                WHERE rc.rwgps_user_id = $1
                "#,
                    rwgps_user_id as i32
                )
                .fetch_all(conn.as_mut())
                .await?
            }
            UserFilter::Email(email) => {
                sqlx::query_as!(
                    UserRow,
                    r#"
                    SELECT 
                        u.*,
                        rc.id as "rwgps_id?",
                        rc.rwgps_user_id as "rwgps_user_id?",
                        rc.access_token as "rwgps_access_token?",
                        rc.created_at as "rwgps_created_at?",
                        rc.updated_at as "rwgps_updated_at?"
                    FROM users u
                    LEFT JOIN user_rwgps_connections rc ON rc.user_id = u.id
                    WHERE u.email = $1
                    "#,
                    email
                )
                .fetch_all(conn.as_mut())
                .await?
            }
        };

        Ok(users.into_iter().map(User::try_from).collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<User>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            UserRow,
            r#"
            SELECT 
                u.*,
                rc.id as "rwgps_id?",
                rc.rwgps_user_id as "rwgps_user_id?",
                rc.access_token as "rwgps_access_token?",
                rc.created_at as "rwgps_created_at?",
                rc.updated_at as "rwgps_updated_at?"
            FROM users u
            INNER JOIN user_rwgps_connections rc ON rc.user_id = u.id
            "#,
        );

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(User::try_from)
            .collect_result_vec()?)
    }

    async fn get(&self, id: <User as Model>::Id) -> Result<User, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(
            UserRow,
            r#"
            SELECT 
                u.*,
                rc.id as "rwgps_id?",
                rc.rwgps_user_id as "rwgps_user_id?",
                rc.access_token as "rwgps_access_token?",
                rc.created_at as "rwgps_created_at?",
                rc.updated_at as "rwgps_updated_at?"
            FROM users u
            LEFT JOIN user_rwgps_connections rc ON rc.user_id = u.id
            WHERE u.id = $1
            "#,
            id.as_uuid()
        );

        Ok(User::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }

    async fn put(&self, model: User) -> Result<(), PostgresRepoError> {
        let mut tx = self.client.begin().await?;

        // Insert/update user
        sqlx::query!(
            r#"
            INSERT INTO users (
                id,
                username,
                password,
                email,
                created_at
            ) VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                password = EXCLUDED.password,
                email = EXCLUDED.email,
                created_at = EXCLUDED.created_at
            "#,
            Uuid::from(model.id()),
            model.username,
            model.password.to_string(),
            model.email,
            model.created_at,
        )
        .execute(tx.as_mut())
        .await?;

        // Handle RWGPS connection
        if let Some(rwgps) = model.rwgps_connection {
            sqlx::query!(
                r#"
                INSERT INTO user_rwgps_connections (
                    id,
                    user_id,
                    rwgps_user_id,
                    access_token,
                    created_at,
                    updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (user_id) DO UPDATE SET
                    rwgps_user_id = EXCLUDED.rwgps_user_id,
                    access_token = EXCLUDED.access_token,
                    updated_at = EXCLUDED.updated_at
                "#,
                rwgps.id,
                rwgps.user_id.as_uuid(),
                rwgps.rwgps_user_id,
                rwgps.access_token,
                rwgps.created_at,
                rwgps.updated_at,
            )
            .execute(tx.as_mut())
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
