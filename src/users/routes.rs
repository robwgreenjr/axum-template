use std::sync::Arc;

use axum::{Json, Router, routing::get};
use axum::extract::State;
use axum::http::StatusCode;
use sea_orm::{EntityTrait, PaginatorTrait};

use crate::AppState;
use crate::database::query_builder::QueryBuilder;
use crate::global::parameter_query_builder::ParameterQueryBuilder;
use crate::global::response_builder::DataListResponse;
use crate::users::user;

// TODO: Finish all user routes
pub async fn find_all(
    state: State<Arc<AppState>>,
    ParameterQueryBuilder(parameter_query_result): ParameterQueryBuilder,
) -> Result<Json<DataListResponse<user::Model>>, (StatusCode, Json<DataListResponse<user::Model>>)> {
    let users: Vec<user::Model> = QueryBuilder::get_list::<user::Entity>(&state.db, parameter_query_result).await;

    let count = user::Entity::find()
        .count(&state.db)
        .await
        .expect("Cannot count users");

    let data: DataListResponse<user::Model> = DataListResponse::init::<user::Entity>(&state.db, users).await;

    DataListResponse::respond(&data)
}

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(find_all))
}