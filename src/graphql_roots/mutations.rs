use crate::db::{establish_connection};
use crate::schema::todos::{Todo, TodoInput};
use diesel::prelude::*;
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_todo(data: TodoInput) -> FieldResult<Todo> {
        use crate::models::todos::todos::dsl;

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
        use crate::models::todos::todos::dsl;

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
        use crate::models::todos::todos::dsl;

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