use async_graphql::dataloader::Loader;
use howitt::models::user::{User, UserFilter, UserId};
use howitt::repos::UserRepo;
use std::{collections::HashMap, sync::Arc};

pub struct UserLoader {
    user_repo: UserRepo,
}

impl UserLoader {
    pub fn new(user_repo: UserRepo) -> Self {
        Self { user_repo }
    }
}

impl Loader<UserId> for UserLoader {
    type Value = User;
    type Error = Arc<anyhow::Error>;

    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, Self::Value>, Self::Error> {
        let users = self
            .user_repo
            .filter_models(UserFilter::Ids(keys.to_vec()))
            .await
            .map_err(|e| Arc::new(e))?;

        Ok(users.into_iter().map(|user| (user.id, user)).collect())
    }
}
