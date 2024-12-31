use async_graphql::{Context, Enum};

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    SuperUser,
    Public,
}

impl Role {
    pub async fn from_context<'ctx>(_ctx: &Context<'ctx>) -> Result<Role, async_graphql::Error> {
        // let RequestData { credentials } = ctx.data()?;

        Ok(Role::Public)
    }
}
