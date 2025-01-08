use crate::schema::todos::{Todo};
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::graphql_roots::Context;

pub struct SubscriptionRoot;

#[juniper::graphql_object]
#[graphql(context = Context)]
impl SubscriptionRoot {
    fn dummy_field() -> FieldResult<String> {
        Ok("dummy".into())
    }
}