use diesel::prelude::*;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;



use crate::db;
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
        let mut conn = db::establish_connection();
        use crate::models::todos::todos::dsl::{todos, id};

        // Query the database for todos with the given IDs
        let results = todos
            .filter(id.eq_any(ids))
            .load::<Todo>(&mut conn)?;

        // Convert the results into a HashMap
        let todo_map = results.into_iter().map(|todo| (todo.id, todo)).collect();

        Ok(todo_map)
    }
}


pub struct TodoBatcher {
    pub repo: Repository,
}

impl BatchFn<i32, Result<Todo, Arc<anyhow::Error>>> for TodoBatcher {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Result<Todo, Arc<anyhow::Error>>> {
        match self.repo.load_todos_by_ids(keys).await {
            Ok(found_todos) => found_todos.into_iter().map(|(id, todo)| (id, Ok(todo))).collect(),
            Err(e) => {
                // Since `anyhow::Error` doesn't implement `Clone`, we have to
                // work around here.
                let e = Arc::new(e);
                keys.iter().map(|k| (k.clone(), Err(e.clone()))).collect()
            }
        }
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