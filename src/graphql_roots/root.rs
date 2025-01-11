use juniper::{EmptySubscription, RootNode};

use crate::graphql_roots::queries;
use crate::graphql_roots::mutations;

pub type Schema = RootNode<'static, queries::QueryRoot, mutations::MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(queries::QueryRoot {}, mutations::MutationRoot {}, EmptySubscription::new())
}
