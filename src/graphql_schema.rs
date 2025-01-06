extern crate dotenv;

use crate::models::{Todo, TodoInput};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn todos() -> FieldResult<Vec<Todo>> {
        use crate::schema::todos::dsl;

        let mut connection = establish_connection();
        let results = dsl::todos.load::<Todo>(&mut connection);
        match results {
            Ok(todos) => Ok(todos),
            Err(_) => Err(FieldError::new(
                "Error loading todos",
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }
    fn todo(id: i32) -> FieldResult<Todo> {
        use crate::schema::todos::dsl;

        let mut connection = establish_connection();
        let results = dsl::todos.filter(dsl::id.eq(id)).first::<Todo>(&mut connection);
        match results {
            Ok(todo) => Ok(todo),
            Err(_) => Err(FieldError::new(
                "Todo not found",
                graphql_value!({ "code": "BAD_USER_INPUT" }),
            )),
        }
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_todo(data: TodoInput) -> FieldResult<Todo> {
        use crate::schema::todos::dsl;

        let mut connection = establish_connection();
        let results = diesel::insert_into(dsl::todos)
            .values(&data)
            .get_result::<Todo>(&mut connection);
        match results {
            Ok(todo) => Ok(todo),
            Err(_) => Err(FieldError::new(
                "Error creating todo",
                graphql_value!({ "code": "BAD_USER_INPUT" }),
            )),
        }
    }
    fn update_todo(id: i32, data: TodoInput) -> FieldResult<Todo> {
        use crate::schema::todos::dsl;

        let mut connection = establish_connection();
        let results = diesel::update(dsl::todos.find(id))
            .set(&data)
            .get_result::<Todo>(&mut connection);
        match results {
            Ok(todo) => Ok(todo),
            Err(_) => Err(FieldError::new(
                "Error updating todo",
                graphql_value!({ "code": "BAD_USER_INPUT" }),
            )),
        }
    }
    fn delete_todo(id: i32) -> FieldResult<Todo> {
        use crate::schema::todos::dsl;

        let mut connection = establish_connection();
        let results = diesel::delete(dsl::todos.find(id)).get_result::<Todo>(&mut connection);
        match results {
            Ok(todo) => Ok(todo),
            Err(_) => Err(FieldError::new(
                "Error deleting todo",
                graphql_value!({ "code": "BAD_USER_INPUT" }),
            )),
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
