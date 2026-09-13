use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use howitt::services::user::auth::Login;

use crate::graphql::{context::RequestData, schema::Schema};

pub async fn graphql_handler(
    State(schema): State<Schema>,
    login: Option<Login>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    request = request.data(RequestData { login });
    schema.execute(request).await.into()
}

pub async fn graphiql_handler() -> impl IntoResponse {
    Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/")
            .version("4.1.2")
            .finish()
            .replace(
                "https://unpkg.com/react@18/",
                "https://unpkg.com/react@18.3.1/",
            )
            .replace(
                "https://unpkg.com/react-dom@18/",
                "https://unpkg.com/react-dom@18.3.1/",
            ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn graphiql_pins_compatible_script_and_stylesheet_versions() {
        let response = graphiql_handler().await.into_response();
        let body = axum::body::to_bytes(response.into_body(), 64 * 1024)
            .await
            .unwrap();
        let html = std::str::from_utf8(&body).unwrap();
        for asset in [
            "graphiql@4.1.2/graphiql.min.js",
            "graphiql@4.1.2/graphiql.min.css",
            "react@18.3.1/umd/react.development.js",
            "react-dom@18.3.1/umd/react-dom.development.js",
        ] {
            assert!(
                html.contains(&format!("https://unpkg.com/{asset}")),
                "Missing pinned asset: {asset}"
            );
        }
        assert!(!html.contains("https://unpkg.com/graphiql@4/"));
        assert!(html.contains("createUrl('/')"));
    }
}
