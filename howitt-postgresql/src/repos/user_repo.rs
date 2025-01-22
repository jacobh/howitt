use argon2::password_hash::Encoding;
use argon2::PasswordHash;
use chrono::{DateTime, Utc};
use howitt::ext::iter::ResultIterExt;

use howitt::models::user::{UserFilter, UserId};
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
}

impl TryFrom<UserRow> for User {
    type Error = PostgresRepoError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: UserId::from(row.id),
            username: row.username,
            password: PasswordHash::parse(&row.password, Encoding::default())
                .unwrap()
                .serialize(),
            email: row.email,
            created_at: row.created_at,
            linked_accounts: vec![],
        })
    }
}

#[derive(Debug, derive_more::Constructor)]
pub struct PostgresUserRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresUserRepo {
    type Model = User;
    type Error = PostgresRepoError;

    async fn filter_models(&self, filter: UserFilter) -> Result<Vec<User>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = match filter {
            UserFilter::Username(username) => sqlx::query_as!(
                UserRow,
                r#"select * from users where username = $1 or $1 is null"#,
                username
            ),
        };

        Ok(query
            .fetch_all(conn.as_mut())
            .await?
            .into_iter()
            .map(User::try_from)
            .collect_result_vec()?)
    }

    async fn all_indexes(&self) -> Result<Vec<<User as Model>::IndexItem>, PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query_as!(UserRow, r#"select * from users"#);

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
            r#"select * from users where id = $1"#,
            id.as_uuid()
        );

        Ok(User::try_from(query.fetch_one(conn.as_mut()).await?)?)
    }
    async fn get_index(
        &self,
        id: <User as Model>::Id,
    ) -> Result<<User as Model>::IndexItem, PostgresRepoError> {
        self.get(id).await
    }
    async fn put(&self, model: User) -> Result<(), PostgresRepoError> {
        let mut conn = self.client.acquire().await.unwrap();

        let query = sqlx::query!(
            r#"insert into users (
                id,
                username,
                password,
                email,
                created_at
            ) values ($1, $2, $3, $4, $5)
             "#,
            Uuid::from(model.id()),
            model.username,
            model.password.to_string(),
            model.email,
            Utc::now(),
        );

        query.execute(conn.as_mut()).await?;

        Ok(())
    }
}
