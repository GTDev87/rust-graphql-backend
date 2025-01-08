use crate::models::todos::todos;
use diesel::{AsChangeset, Insertable};
use juniper::{GraphQLInputObject};
use crate::graphql_roots::Context;

#[derive(GraphQLInputObject, Insertable, AsChangeset)]
#[table_name = "todos"]
pub struct TodoInput {
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Queryable, Debug, Clone)]
pub struct Todo {
    id: i32,
    title: String,
    description: String,
    done: bool,
}


#[juniper::graphql_object]
#[graphql(context = Context)]
impl Todo {
    fn id(&self) -> i32 {
        self.id
    }
    fn title(&self) -> String {
        self.title.to_string()
    }
    fn description(&self) -> String {
        self.description.to_string()
    }
    fn done(&self) -> bool {
        self.done
    }
}