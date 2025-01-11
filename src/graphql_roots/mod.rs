use juniper::{RootNode};
use std::collections::HashMap;
use std::sync::Arc;
use dataloader::non_cached::Loader;
use crate::db;
use diesel::prelude::*;

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
    pub async fn load_todos_by_ids(&self, ids: &[crate::schema::todos::TodoId]) -> anyhow::Result<HashMap<crate::schema::todos::TodoId, crate::models::todos::Todo>> { 
        use crate::models::todos::todos::dsl::{todos, id};
        
        let mut conn = db::establish_connection();

        // Query the database for todos with the given IDs
        let results = todos
            .filter(id.eq_any(ids))
            .load::<crate::models::todos::Todo>(&mut conn)?;

        // Convert the results into a HashMap
        let todo_map = results.into_iter().map(|todo| (todo.id, todo)).collect();

        Ok(todo_map)
    }
}

#[derive(Clone)]
pub struct Context {
    repo: Repository,
    pub todo_loader: crate::schema::todos::TodoLoader,
}



impl juniper::Context for Context {}

pub fn create_context() -> Context {
    let repo = Repository;
    let todo_loader = crate::schema::todos::new_todo_loader(repo.clone());
    Context { repo, todo_loader }
}