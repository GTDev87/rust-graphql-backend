use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};

pub mod queries;
pub mod mutations;

pub type Schema = RootNode<'static, queries::QueryRoot, mutations::MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(queries::QueryRoot {}, mutations::MutationRoot {}, EmptySubscription::new())
}
