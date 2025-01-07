#[macro_use]
extern crate diesel;

pub mod db;
mod graphql_roots;
pub mod models;
pub mod schema;

use crate::graphql_roots::{create_schema, Schema};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest, GraphQLResponse};
use rocket::{get, post, routes, State};
use rocket::serde::json::Json;

use rocket::fairing::AdHoc;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::sync::Arc;

/// GraphiQL playground UI
#[get("/graphiql")]
fn graphql_playground() -> RawHtml<String> {
    RawHtml(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[post("/graphql", data = "<request>")]
async fn graphql(schema: &State<Arc<Schema>>, request: Json<GraphQLRequest>) -> Json<GraphQLResponse> {
    Json(
        request
            .into_inner()
            .execute(&schema, &())
            .await
    )
}


#[rocket::launch]
fn rocket() -> _ {
    let schema = Arc::new(create_schema());

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                rocket::http::Method::Get,
                rocket::http::Method::Post
            ]
            .into_iter()
            .map(From::from)
            .collect())
        .to_cors()
        .expect("Cors setup failed");

    rocket::build()
        .mount("/", routes![graphql_playground, graphql])
        .manage(schema)
        .attach(cors)
        .attach(AdHoc::on_ignite("Cors", |rocket| async { rocket }))
}