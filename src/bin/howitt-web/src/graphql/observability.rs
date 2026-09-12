//! Resolver-only telemetry: schema metadata, never arguments, aliases or values.
use async_graphql::{
    extensions::{
        Extension, ExtensionContext, ExtensionFactory, NextExecute, NextResolve, ResolveInfo,
    },
    QueryPathSegment, Response, ServerResult, Value,
};
use std::sync::Arc;

pub struct ResolverTracing;

impl ExtensionFactory for ResolverTracing {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(Self)
    }
}

#[async_trait::async_trait]
impl Extension for ResolverTracing {
    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        // Anchor the entire Rust execution future, including sibling resolvers,
        // independently of the JS context that happens to wake its task queue.
        let span = howitt_observability::TraceSpan::new("graphql.execute", "graphql.outcome");
        let response = span.scope(next.run(ctx, operation_name)).await;
        span.complete(if response.errors.is_empty() {
            "ok"
        } else {
            "error"
        });
        response
    }

    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<Value>> {
        // async-graphql also invokes this hook for every list element. Those
        // are output serialization, not field resolvers (e.g. GPS coordinates).
        if info.is_for_introspection
            || info.name.starts_with("__")
            || matches!(info.path_node.segment, QueryPathSegment::Index(_))
        {
            return next.run(ctx, info).await;
        }

        let span = howitt_observability::TraceSpan::new(
            &format!("graphql.resolve {}.{}", info.parent_type, info.name),
            "graphql.outcome",
        );
        span.attribute("graphql.parent_type", info.parent_type);
        span.attribute("graphql.field.name", info.name);
        span.attribute("graphql.return_type", info.return_type);
        span.trace(next.run(ctx, info)).await
    }
}
