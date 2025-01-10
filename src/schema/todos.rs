use crate::models::todos::todos;
use diesel::{AsChangeset, Insertable};
use juniper::{GraphQLInputObject};
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
    // fn title(&self) -> String {
    //     // self.title.to_string()
    //     String::from("hello title")
    // }
    // fn description(&self) -> String {
    //     // self.description.to_string()
    //     String::from("hello description")
    // }
    // fn done(&self) -> bool {
    //     // self.done
    //     false
    // }
}