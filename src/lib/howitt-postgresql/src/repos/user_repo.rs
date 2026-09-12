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

impl TryFrom<&tokio_postgres::Row> for UserRow {
    type Error = tokio_postgres::Error;

    fn try_from(row: &tokio_postgres::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            email: row.try_get("email")?,
            created_at: row.try_get("created_at")?,
            rwgps_id: row.try_get("rwgps_id")?,
            rwgps_user_id: row.try_get("rwgps_user_id")?,
            rwgps_access_token: row.try_get("rwgps_access_token")?,
            rwgps_created_at: row.try_get("rwgps_created_at")?,
            rwgps_updated_at: row.try_get("rwgps_updated_at")?,
        })
    }
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
        let conn = self.client.acquire().await?;

        let users = match filter {
            UserFilter::Ids(ids) => {
                let uuids: Vec<_> = ids.into_iter().map(Uuid::from).collect();

                conn.query(
                    r#"
                    SELECT
                        u.*,
                        rc.id as "rwgps_id",
                        rc.rwgps_user_id as "rwgps_user_id",
                        rc.access_token as "rwgps_access_token",
                        rc.created_at as "rwgps_created_at",
                        rc.updated_at as "rwgps_updated_at"
                    FROM users u
                    LEFT JOIN user_rwgps_connections rc ON rc.user_id = u.id
                    WHERE u.id = ANY($1)
                    "#,
                    &[&uuids],
                )
                .await?
                .iter()
                .map(UserRow::try_from)
                .collect::<Result<Vec<_>, _>>()?
            }
            UserFilter::Username(username) => conn
                .query(
                    r#"
                    SELECT
                        u.*,
                        rc.id as "rwgps_id",
                        rc.rwgps_user_id as "rwgps_user_id",
                        rc.access_token as "rwgps_access_token",
                        rc.created_at as "rwgps_created_at",
                        rc.updated_at as "rwgps_updated_at"
                    FROM users u
                    LEFT JOIN user_rwgps_connections rc ON rc.user_id = u.id
                    WHERE u.username = $1
                    "#,
                    &[&(username)],
                )
                .await?
                .iter()
                .map(UserRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            UserFilter::RwgpsId(rwgps_user_id) => conn
                .query(
                    r#"
                SELECT
                    u.*,
                    rc.id as "rwgps_id",
                    rc.rwgps_user_id as "rwgps_user_id",
                    rc.access_token as "rwgps_access_token",
                    rc.created_at as "rwgps_created_at",
                    rc.updated_at as "rwgps_updated_at"
                FROM users u
                INNER JOIN user_rwgps_connections rc ON rc.user_id = u.id
                WHERE rc.rwgps_user_id = $1
                "#,
                    &[&(rwgps_user_id as i32)],
                )
                .await?
                .iter()
                .map(UserRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            UserFilter::Email(email) => conn
                .query(
                    r#"
                    SELECT
                        u.*,
                        rc.id as "rwgps_id",
                        rc.rwgps_user_id as "rwgps_user_id",
                        rc.access_token as "rwgps_access_token",
                        rc.created_at as "rwgps_created_at",
                        rc.updated_at as "rwgps_updated_at"
                    FROM users u
                    LEFT JOIN user_rwgps_connections rc ON rc.user_id = u.id
                    WHERE u.email = $1
                    "#,
                    &[&(email)],
                )
                .await?
                .iter()
                .map(UserRow::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        };

        Ok(users.into_iter().map(User::try_from).collect_result_vec()?)
    }

    async fn all(&self) -> Result<Vec<User>, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        Ok(conn
            .query(
                r#"
            SELECT
                u.*,
                rc.id as "rwgps_id",
                rc.rwgps_user_id as "rwgps_user_id",
                rc.access_token as "rwgps_access_token",
                rc.created_at as "rwgps_created_at",
                rc.updated_at as "rwgps_updated_at"
            FROM users u
            INNER JOIN user_rwgps_connections rc ON rc.user_id = u.id
            "#,
                &[],
            )
            .await?
            .iter()
            .map(UserRow::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(User::try_from)
            .collect_result_vec()?)
    }

    async fn get(&self, id: <User as Model>::Id) -> Result<User, PostgresRepoError> {
        let conn = self.client.acquire().await?;

        Ok(User::try_from(UserRow::try_from(
            &conn
                .query_one(
                    r#"
            SELECT
                u.*,
                rc.id as "rwgps_id",
                rc.rwgps_user_id as "rwgps_user_id",
                rc.access_token as "rwgps_access_token",
                rc.created_at as "rwgps_created_at",
                rc.updated_at as "rwgps_updated_at"
            FROM users u
            LEFT JOIN user_rwgps_connections rc ON rc.user_id = u.id
            WHERE u.id = $1
            "#,
                    &[&(id.as_uuid())],
                )
                .await?,
        )?)?)
    }

    async fn put(&self, model: User) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await?;
        let tx = conn.transaction().await?;

        // Insert/update user
        tx.execute(
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
            &[
                &Uuid::from(model.id()),
                &model.username,
                &model.password.to_string(),
                &model.email,
                &model.created_at,
            ],
        )
        .await?;

        // Handle RWGPS connection
        if let Some(rwgps) = model.rwgps_connection {
            tx.execute(
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
                &[
                    &rwgps.id,
                    rwgps.user_id.as_uuid(),
                    &rwgps.rwgps_user_id,
                    &rwgps.access_token,
                    &rwgps.created_at,
                    &rwgps.updated_at,
                ],
            )
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
