use juniper::{EmptySubscription, RootNode};

mod queries;
mod mutations;

pub type Schema = RootNode<'static, queries::QueryRoot, mutations::MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(queries::QueryRoot {}, mutations::MutationRoot {}, EmptySubscription::new())
}
