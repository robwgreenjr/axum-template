use axum::{Router, routing::{get}};

pub fn user_routes() -> Router {
    Router::new().route("/users", get(|| async { "Users Endpoint!" }))
}