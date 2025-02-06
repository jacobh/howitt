use chrono::Utc;
use clap::{Args, Subcommand};
use howitt::{
    models::user::{User, UserId},
    repos::Repo,
    services::user::{auth::UserAuthService, password::hash_password},
};
use howitt_postgresql::PostgresRepos;
use std::sync::Arc;

use crate::Context;

#[derive(Subcommand)]
pub enum UserCommands {
    Create,
    List,
    Login,
    VerifyToken(TokenArgs),
}

#[derive(Args)]
pub struct TokenArgs {
    token: String,
}

pub async fn handle(
    command: &UserCommands,
    Context {
        repos: PostgresRepos { user_repo, .. },
        ..
    }: Context,
) -> Result<(), anyhow::Error> {
    match command {
        UserCommands::Create => {
            let username = inquire::Text::new("username").prompt()?;
            let email = inquire::Text::new("email").prompt()?;
            let password = inquire::Password::new("password").prompt()?;
            let created_at = Utc::now();

            let password = hash_password(&password)?;

            let user = User {
                id: UserId::from_datetime(created_at),
                username,
                email,
                password,
                created_at,
                rwgps_connection: None,
            };

            user_repo.put(user).await?;

            dbg!("done");
        }
        UserCommands::List => {
            let users = user_repo.all().await?;
            dbg!(users);
        }
        UserCommands::Login => {
            let service = UserAuthService::new(Arc::new(user_repo), String::from("asdf123"));

            let username = inquire::Text::new("username").prompt()?;
            let password = inquire::Password::new("password")
                .without_confirmation()
                .prompt()?;

            let res = service.login(&username, &password).await;

            let _ = dbg!(res)?;
        }
        UserCommands::VerifyToken(TokenArgs { token }) => {
            let service = UserAuthService::new(Arc::new(user_repo), String::from("asdf123"));

            let res = service.verify(token).await;

            let _ = dbg!(res)?;
        }
    }

    Ok(())
}
