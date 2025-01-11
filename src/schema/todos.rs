use crate::models::todos::todos;
use diesel::{AsChangeset, Insertable};
use juniper::{GraphQLInputObject, FieldResult};
use crate::graphql_roots::Context;
use std::sync::Arc;
use crate::graphql_roots::{Repository};
use std::collections::HashMap;
use dataloader::non_cached::Loader;

pub type TodoId = i32;

#[derive(GraphQLInputObject, Insertable, AsChangeset)]
#[table_name = "todos"]
pub struct TodoInput {
    pub title: String,
    pub description: String,
    pub done: bool,
}

// todo object with only id full data will be queried on impl
#[derive(Queryable, Debug, Clone)]
pub struct Todo {
    pub id: TodoId,
    // title: String,
    // description: String,
    // done: bool,
}

pub struct TodoBatcher {
    repo: Repository,
}

// Since `BatchFn` doesn't provide any notion of fallible loading, like 
// `try_load()` returning `Result<HashMap<K, V>, E>`, we handle possible
// errors as loaded values and unpack them later in the resolver.
impl dataloader::BatchFn<TodoId, Result<crate::models::todos::Todo, Arc<anyhow::Error>>> for TodoBatcher {
    async fn load(
        &mut self, 
        ids: &[TodoId],
    ) -> HashMap<TodoId, Result<crate::models::todos::Todo, Arc<anyhow::Error>>> {
        // Effectively performs the following SQL query:
        // SELECT id, name FROM cults WHERE id IN (${cult_id1}, ${cult_id2}, ...)
        match self.repo.load_todos_by_ids(ids).await {
            Ok(found_todos) => {
                found_todos.into_iter().map(|(id, todo)| (id, Ok(todo))).collect()
            }
            // One could choose a different strategy to deal with fallible loads,
            // like consider values that failed to load as absent, or just panic.
            // See cksac/dataloader-rs#35 for details:
            // https://github.com/cksac/dataloader-rs/issues/35
            Err(e) => {
                // Since `anyhow::Error` doesn't implement `Clone`, we have to
                // work around here.
                let e = Arc::new(e);
                ids.iter().map(|k| (k.clone(), Err(e.clone()))).collect()
            }
        }
    }
}

pub type TodoLoader = Loader<TodoId, Result<crate::models::todos::Todo, Arc<anyhow::Error>>, TodoBatcher>;

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

#[juniper::graphql_object]
#[graphql(context = Context)]
impl Todo {
    fn id(&self) -> i32 {
        self.id
    }
    async fn title(&self, ctx: &Context) -> FieldResult<String> {
        let todo = ctx.todo_loader.try_load(self.id).await??; // Unwrap the Result
        Ok(todo.title)
    }
    async fn description(&self, ctx: &Context) -> FieldResult<String> {
        let todo = ctx.todo_loader.try_load(self.id).await??; // Unwrap the Result
        Ok(todo.description)
    }
    async fn done(&self, ctx: &Context) -> FieldResult<bool> {
        let todo = ctx.todo_loader.try_load(self.id).await??; // Unwrap the Result
        Ok(todo.done)
    }
}