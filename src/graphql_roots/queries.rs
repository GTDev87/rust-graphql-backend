use crate::db;

use crate::schema::todos::{Todo, TodoInput};
use diesel::prelude::*;
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn todos() -> FieldResult<Vec<Todo>> {
        use crate::models::todos::todos::dsl;

        let mut connection = db::establish_connection();
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
        use crate::models::todos::todos::dsl;

        let mut connection = db::establish_connection();
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