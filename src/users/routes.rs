use std::sync::Arc;

use axum::{Json, Router, routing::get};
use axum::extract::State;
use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::database::query_builder::QueryBuilder;
use crate::global::parameter_query_builder::ParameterQueryBuilder;
use crate::users::user;

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    meta: MetaData,
    data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    timestamp: String,
    count: u64,
    page: u64,
    page_count: u64,
    limit: u64,
    cursor: String,
    next: u64,
    previous: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MetaQueryData {}

pub async fn find_all(
    state: State<Arc<AppState>>,
    ParameterQueryBuilder(parameter_query_result): ParameterQueryBuilder,
) -> Result<Json<DataResponse<user::Model>>, (StatusCode, &'static str)> {
    let limit = 200;

    let users: Vec<user::Model> = QueryBuilder::get_list::<user::Entity>(&state.db, parameter_query_result).await;

    let count = user::Entity::find()
        .count(&state.db)
        .await
        .expect("Cannot count users");
    let remaining_count = user::Entity::find()
        .order_by_asc(user::Column::Id)
        .filter(user::Column::Id.gte(400))
        .count(&state.db)
        .await
        .expect("Cannot find users");
    let page_count = count / limit;

    let custom_query = user::Entity::find()
        .order_by_asc(user::Column::Id);
    let mut custom_query_cloned = custom_query.clone();
    custom_query_cloned = custom_query.clone()
                                      .filter(user::Column::FirstName.contains("Ri"));

    let result = custom_query_cloned
        .all(&state.db).await
        .expect("Cannot find users");

    let next = user::Entity::find()
        .order_by_asc(user::Column::Id)
        .limit(1)
        .cursor_by(user::Column::Id)
        .after(users.last().unwrap().id)
        .all(&state.db).await
        .expect("Cannot find users");
    let next_id = if !next.is_empty() {
        next.first().unwrap().id
    } else {
        0
    };

    let previous = user::Entity::find()
        .order_by_desc(user::Column::Id)
        .limit(1)
        .cursor_by(user::Column::Id)
        .before(users.first().unwrap().id).all(&state.db).await
        .expect("Cannot find users");
    let previous_id = if !previous.is_empty() {
        previous.first().unwrap().id
    } else {
        0
    };

    let response = DataResponse {
        meta: MetaData {
            timestamp: Utc::now().to_string(),
            count,
            page: ((count - remaining_count) / limit) + 1,
            page_count,
            limit,
            cursor: "id".to_string(),
            next: next_id as u64,
            previous: previous_id as u64,
        },
        data: users,
    };

    Ok(Json::from(response))
}

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(find_all))
}