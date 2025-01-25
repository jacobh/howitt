use super::context::SchemaData;

pub mod interfaces;
pub mod objects;
pub mod scalars;

pub use interfaces::*;
pub use objects::*;
pub use scalars::*;

pub type Schema = async_graphql::Schema<
    query::Query,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
>;

pub fn build_schema(data: SchemaData) -> Schema {
    Schema::build(
        query::Query,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .register_output_type::<ElevationPath>()
    .data(data)
    .finish()
}
