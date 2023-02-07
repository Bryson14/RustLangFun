use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)] // (1)
struct Health { // (2)
    healthy: bool
}

pub(crate) async fn health() -> impl IntoResponse { // (3)
    let health = Health {
        healthy: true
    };

    (StatusCode::OK, Json(health)) // (4)
}