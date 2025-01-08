use juniper::{RootNode};

mod queries;
mod mutations;
mod subscriptions;

pub type Schema = RootNode<'static, queries::QueryRoot, mutations::MutationRoot, subscriptions::SubscriptionRoot>;

pub fn create_schema() -> Schema {
    Schema::new(queries::QueryRoot {}, mutations::MutationRoot {}, subscriptions::SubscriptionRoot {})
}


#[derive(Clone)]
pub struct Context {
}

impl juniper::Context for Context {}


pub fn create_context() -> Context {
    Context {}
}