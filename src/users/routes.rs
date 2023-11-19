use std::sync::Arc;

use axum::{Json, Router, routing::get};
use axum::extract::State;
use axum::http::StatusCode;
use sea_orm::EntityTrait;

use crate::AppState;
use crate::global::parameter_query_builder::ParameterQueryBuilder;
use crate::global::response_builder::{DataListResponse, DataListResponseDto};
use crate::users::user::Model;
use crate::users::user_management::get_all;

// TODO: Finish all user routes
pub async fn find_all(
    state: State<Arc<AppState>>,
    ParameterQueryBuilder(parameter_query_result): ParameterQueryBuilder,
) -> Result<Json<DataListResponseDto<Model>>, (StatusCode, Json<DataListResponseDto<Model>>)> {
    let users = get_all(&state.db, parameter_query_result).await;

    match users {
        Ok(users) => {
            let data: DataListResponse<Model> = DataListResponse::init(Some(users), None).await;

            data.respond()
        }
        Err(errors) => {
            let data: DataListResponse<Model> = DataListResponse::init(None, Some(errors)).await;

            data.respond()
        }
    }
}

// pub async fn find(
//     state: State<Arc<AppState>>,
//     ParameterQueryBuilder(parameter_query_result): ParameterQueryBuilder,
// ) -> Result<Json<DataListResponseDto<Model>>, (StatusCode, Json<DataListResponseDto<Model>>)> {
//
// }

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(find_all))
    // .route("/user", get(find))
}