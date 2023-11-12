use std::sync::Arc;

use axum::{Json, Router, routing::get};
use axum::extract::State;
use axum::http::StatusCode;
use sea_orm::EntityTrait;

use crate::AppState;
use crate::database::query_builder::QueryBuilder;
use crate::global::error_handling::ErrorDetails;
use crate::global::parameter_query_builder::ParameterQueryBuilder;
use crate::global::response_builder::{DataListResponse, DataListResponseDto};
use crate::users::user;
use crate::users::user::Model;

// TODO: Finish all user routes
pub async fn find_all(
    state: State<Arc<AppState>>,
    ParameterQueryBuilder(parameter_query_result): ParameterQueryBuilder,
) -> Result<Json<DataListResponseDto<Model>>, (StatusCode, Json<DataListResponseDto<Model>>)> {
    let users: Result<Vec<Model>, Vec<ErrorDetails>> = QueryBuilder::get_list::<user::Entity>(&state.db, parameter_query_result).await;

    match users {
        Ok(users) => {
            let data: DataListResponse<Model> = DataListResponse::init::<user::Entity>(&state.db, Some(users), None).await;

            data.respond()
        }
        Err(errors) => {
            let data: DataListResponse<Model> = DataListResponse::init::<user::Entity>(&state.db, None, Some(errors)).await;

            data.respond()
        }
    }
}

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(find_all))
}