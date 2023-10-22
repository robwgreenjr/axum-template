use std::env;
use std::sync::Arc;

use axum::Router;
use sea_orm::{Database, DatabaseConnection};

use crate::users::routes::user_routes;

mod users;
mod global;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db: DatabaseConnection = Database::connect(db_url).await.expect("Cannot find posts in page");

    let state = Arc::new(AppState { db });

    let app = Router::new()
        .merge(user_routes())
        .with_state(state);

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let address = host + ":" + &*port;
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}