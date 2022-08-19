use crate::schema::todos;
use diesel::Insertable;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Queryable, GraphQLObject)]
pub struct Todo {
    id: i32,
    title: String,
    description: String,
    done: bool,
}
