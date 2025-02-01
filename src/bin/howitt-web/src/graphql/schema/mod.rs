use super::context::SchemaData;

pub mod interfaces;
pub mod mutation;
pub mod objects;
pub mod query;
pub mod scalars;

pub use interfaces::*;
pub use objects::*;
pub use scalars::*;

pub type Schema =
    async_graphql::Schema<query::Query, mutation::Mutation, async_graphql::EmptySubscription>;

pub fn build_schema(data: SchemaData) -> Schema {
    Schema::build(
        query::Query,
        mutation::Mutation,
        async_graphql::EmptySubscription,
    )
    .register_output_type::<ElevationPath>()
    .register_output_type::<MediaTarget>()
    .register_output_type::<TemporalContentBlock>()
    .data(data)
    .finish()
}
