#[macro_use]
extern crate diesel;

mod graphql_schema;
pub mod models;
pub mod schema;

use crate::graphql_schema::{create_schema, Schema};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use rocket::{get, post, routes, State};
use rocket::fairing::AdHoc;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::sync::Arc;
use serde_json;

/// GraphiQL playground UI
#[get("/graphiql")]
fn graphql_playground() -> RawHtml<String> {
    RawHtml(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[post("/graphql", data = "<request>")]
async fn graphql(schema: &State<Arc<Schema>>, request: Json<GraphQLRequest>) -> Json<juniper::http::GraphQLResponse> {
    // Convert the incoming GraphQLRequest to an owned type
    let request = request.into_inner();
    // Execute the request
    let response = request.execute(&schema, &()).await;
    // Return the response as JSON
    Json(response)
}


#[rocket::launch]
fn rocket() -> _ {
    let schema = Arc::new(create_schema());

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![rocket::http::Method::Get, rocket::http::Method::Post].into_iter().map(From::from).collect())
        .to_cors()
        .expect("Cors setup failed");

    rocket::build()
        .mount("/", routes![graphql_playground, graphql])
        .manage(schema)
        .attach(cors)
        .attach(AdHoc::on_ignite("Cors", |rocket| async { rocket }))
}