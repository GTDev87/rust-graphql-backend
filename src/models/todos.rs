use diesel::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use anyhow::{Result, Error};

use crate::graphql_roots::{Repository};


table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        done -> Bool,
    }
}

#[derive(Queryable, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}


impl Todo {
    pub fn load_todos_by_ids(ids: &[i32]) -> Result<HashMap<i32, Todo>> {
        use crate::models::todos::todos::dsl::{todos, id};

        let load_todos = crate::dataloader::load_by_ids(
            |conn, ids| {todos.filter(id.eq_any(ids)).load::<Todo>(conn).map_err(Error::from)},
            |todo: &Todo| todo.id,
        );

        load_todos(ids)
    }
}

pub struct TodoBatcher {
    pub repo: Repository,
}

impl BatchFn<i32, Result<Todo, Arc<anyhow::Error>>> for TodoBatcher {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Result<Todo, Arc<anyhow::Error>>> {
        let found_todos = self.repo.load_todos_by_ids(keys).await;
        crate::dataloader::handle_found_items(keys, found_todos)
    }
}

pub type TodoLoader = Loader<i32, Result<Todo, Arc<anyhow::Error>>, TodoBatcher>;

pub fn new_todo_loader(repo: Repository) -> TodoLoader {
    TodoLoader::new(TodoBatcher { repo })
        // Usually a `Loader` will coalesce all individual loads which occur 
        // within a single frame of execution before calling a `BatchFn::load()`
        // with all the collected keys. However, sometimes this behavior is not
        // desirable or optimal (perhaps, a request is expected to be spread out
        // over a few subsequent ticks).
        // A larger yield count will allow more keys to be appended to the batch,
        // but will wait longer before the actual load. For more details see:
        // https://github.com/cksac/dataloader-rs/issues/12 
        // https://github.com/graphql/dataloader#batch-scheduling
        .with_yield_count(100)
}