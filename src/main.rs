#[macro_use]
extern crate diesel;

pub mod db;
mod graphql_roots;
pub mod models;
pub mod schema;

use env_logger::Env;

use env_logger;

use log::{info, warn, error};

use crate::graphql_roots::{create_schema, create_context, Schema, Context};
use axum::{
    extract::Extension,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

/// GraphiQL playground UI
async fn graphql_playground() -> Html<String> {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
async fn graphql(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(context): Extension<Context>,
    Json(request): Json<GraphQLRequest>,
) -> Json<juniper::http::GraphQLResponse> {
    info!("Serving GraphiQL playground");

    let response = request.execute(&schema, &context).await;
    Json(response)
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info,diesel=debug")).init();
    // env_logger::init();
    info!("Starting the server...");

    let schema = Arc::new(create_schema());
    let context = create_context();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/graphiql", get(graphql_playground))
        .route("/graphql", post(graphql))
        .layer(cors)
        .layer(Extension(schema))
        .layer(Extension(context));

    let addr = "0.0.0.0:8080".parse::<std::net::SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}