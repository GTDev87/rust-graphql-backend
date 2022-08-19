extern crate dotenv;

use crate::models::{NewTodo, Todo};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use juniper::{EmptySubscription, RootNode};
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn todos() -> Vec<Todo> {
        use crate::schema::todos::dsl::*;

        let connection = establish_connection();
        let results = todos
            .load::<Todo>(&connection)
            .expect("Error loading todos");
        results
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_todo(new_todo: NewTodo) -> Todo {
        use crate::schema::todos::dsl::*;
        let connection = establish_connection();
        let todo = diesel::insert_into(todos)
            .values(&new_todo)
            .get_result::<Todo>(&connection)
            .expect("Error saving new todo");
        todo
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
