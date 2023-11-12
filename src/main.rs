use std::env;
use std::sync::Arc;

use axum::Router;
use sea_orm::{Database, DatabaseConnection};

use crate::users::routes::user_routes;

mod tests;
mod users;
mod global;
mod database;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let address = host + ":" + &*port;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    axum::Server::bind(&address.parse().unwrap())
        .serve(app(db_url).await.into_make_service())
        .await
        .unwrap();
}

pub async fn app(db_url: String) -> Router {
    let db: DatabaseConnection = Database::connect(db_url).await.expect("Cannot find posts in page");
    let state = Arc::new(AppState { db });

    Router::new()
        .merge(user_routes())
        .with_state(state)
}