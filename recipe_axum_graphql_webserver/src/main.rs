use crate::routes::health;
use axum::{routing::get, Router, Server};

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}