use juniper::RootNode;
use std::collections::HashMap;
use anyhow::Result;

mod queries;
mod mutations;
mod subscriptions;

pub type Schema = RootNode<'static, queries::QueryRoot, mutations::MutationRoot, subscriptions::SubscriptionRoot>;

pub fn create_schema() -> Schema {
    Schema::new(queries::QueryRoot {}, mutations::MutationRoot {}, subscriptions::SubscriptionRoot {})
}

#[derive(Clone)]
pub struct Repository;

impl Repository {
    pub async fn load_todos_by_ids(&self, ids: &[i32]) -> Result<HashMap<i32, crate::models::todos::Todo>> {
        crate::models::todos::Todo::load_todos_by_ids(ids)
    }
}

#[derive(Clone)]
pub struct Context {
    pub repo: Repository,
    pub todo_loader: crate::models::todos::TodoLoader,
}

impl juniper::Context for Context {}

pub fn create_context() -> Context {
    let repo = Repository;
    let todo_loader = crate::models::todos::new_todo_loader(repo.clone());
    Context {
        repo,
        todo_loader,
    }
}