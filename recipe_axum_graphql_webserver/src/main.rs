use crate::model::QueryRoot;
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};

mod model;
mod routes;

#[tokio::main]
async fn main() {
    // You now need to build your schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        // Both routes are registered here
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health))
        // You need to make the schema available to your route handlers
        .layer(Extension(schema)); // (1)

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}