use crate::schema::todos;
use diesel::{AsChangeset, Insertable};
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLInputObject, Insertable, AsChangeset)]
#[table_name = "todos"]
pub struct TodoInput {
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
