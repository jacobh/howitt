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
            .finish()
            // This template uses React 17 and the GraphiQL UMD global. Newer
            // GraphiQL releases no longer provide that bundle at the same path.
            .replace(
                "https://unpkg.com/graphiql/",
                "https://unpkg.com/graphiql@2.4.7/",
            )
            .replace(
                "https://unpkg.com/react@17/",
                "https://unpkg.com/react@17.0.2/",
            )
            .replace(
                "https://unpkg.com/react-dom@17/",
                "https://unpkg.com/react-dom@17.0.2/",
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
            "graphiql@2.4.7/graphiql.min.js",
            "graphiql@2.4.7/graphiql.min.css",
            "react@17.0.2/umd/react.development.js",
            "react-dom@17.0.2/umd/react-dom.development.js",
        ] {
            assert!(
                html.contains(&format!("https://unpkg.com/{asset}")),
                "Missing pinned asset: {asset}"
            );
        }
        assert!(!html.contains("https://unpkg.com/graphiql/"));
        assert!(html.contains("createUrl('/')"));
    }
}
