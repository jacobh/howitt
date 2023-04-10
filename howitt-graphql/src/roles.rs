use async_graphql::{Context, Enum};
use howitt::models::config::ConfigId;

use crate::{
    context::{RequestData, SchemaData},
    credentials::Credentials,
};

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    SuperUser,
    Public,
}

impl Role {
    pub async fn from_context<'ctx>(ctx: &Context<'ctx>) -> Result<Role, async_graphql::Error> {
        let SchemaData { config_repo, .. } = ctx.data()?;
        let RequestData { credentials } = ctx.data()?;

        match credentials {
            Some(Credentials::Key(key)) => {
                let config = config_repo.get(ConfigId).await?;
                if config.api_keys.contains(key) {
                    Ok(Role::SuperUser)
                } else {
                    Ok(Role::Public)
                }
            }
            None => Ok(Role::Public),
        }
    }
}
