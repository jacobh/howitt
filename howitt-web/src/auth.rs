use howitt::{repos::UserRepo, services::user::auth::UserAuthService};
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct LoginParams {
    username: String,
    password: String,
}

pub fn login_route(
    user_repo: UserRepo,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let auth_service = UserAuthService::new(user_repo, "asdf123".to_string());

    warp::path!("auth" / "login")
        .and(warp::post())
        .and(warp::any().map(move || auth_service.clone()))
        .and(warp::body::json::<LoginParams>())
        .then(
            async move |auth_service: UserAuthService,
                        LoginParams { username, password }|
                        -> Result<_, String> {
                let res = auth_service.clone().login(&username, &password).await;

                match res {
                    Ok(Ok(login)) => Ok(warp::reply::json(&login)),
                    _ => Ok(warp::reply::json(
                        &serde_json::json!({"err": "login failed"}),
                    )),
                }
            },
        )
}
