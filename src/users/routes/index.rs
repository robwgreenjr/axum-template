use std::sync::Arc;

use axum::{Json, Router, routing::get};
use axum::extract::State;
use axum::http::StatusCode;
use sea_orm::{EntityTrait, QuerySelect};

use crate::AppState;
use crate::users::entities::user;

pub async fn find_all(state: State<Arc<AppState>>) -> Result<Json<Vec<user::Model>>, (StatusCode, &'static str)> {
    let users: Vec<user::Model> = user::Entity::find().limit(200).all(&state.db).await.expect("Cannot find posts in page");

    Ok(Json::from(users))
}

pub fn user_routes(State(state): State<Arc<AppState>>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(find_all)).with_state(state)
}