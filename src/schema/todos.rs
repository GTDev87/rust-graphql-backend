use crate::models::todos::todos;
use diesel::{AsChangeset, Insertable};
use juniper::{GraphQLInputObject, FieldResult};
use crate::graphql_roots::Context;

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