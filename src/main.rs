mod users;

use axum::{
    Router,
};
use crate::users::routes::user_routes;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(user_routes());

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}