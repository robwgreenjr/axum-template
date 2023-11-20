use std::collections::HashMap;

use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};

use crate::database::query_builder::{QueryBuilder, QueryResult};
use crate::global::error_handling::ErrorDetails;
use crate::global::parameter_query_builder::{ParameterQueryResult, QuerySort};
use crate::global::response_builder::MetaListData;
use crate::users::user::{Entity, Model};

pub async fn get_all(
    db: &DatabaseConnection,
    mut query_result: ParameterQueryResult,
) -> Result<QueryResult<Model>, Vec<ErrorDetails>> {
    let result: Result<Vec<Model>, Vec<ErrorDetails>> = QueryBuilder::get_list::<Entity>(db, query_result.clone()).await;

    let mut users: Vec<Model> = vec![];
    match result {
        Ok(result) => {
            users = result;
        }
        Err(errors) => {
            return Err(errors);
        }
    }

    // Get current page and total page count
    let original_query = query_result.clone();
    let remaining_count = QueryBuilder::generate(Entity::find(), original_query.clone())
        .count(db)
        .await
        .expect("Cannot count users");

    query_result.remove_cursor();
    let total_count = QueryBuilder::generate(Entity::find(), query_result.clone())
        .count(db)
        .await
        .expect("Cannot count users");

    // Get next and previous cursors
    let mut next_query = original_query.clone();
    next_query.limit += 1;
    let next_result = QueryBuilder::generate(Entity::find(), next_query)
        .all(db).await
        .expect("Cannot find users");

    let next = if !next_result.is_empty() && users.len() as u64 == query_result.limit {
        next_result.last().unwrap().id
    } else {
        0
    };

    let mut previous_query = original_query.clone();
    let mut desc_sort = HashMap::new();
    desc_sort.insert(QuerySort::DESC, vec!["id".to_string()]);
    previous_query.sort_list = desc_sort;

    previous_query.remove_cursor();
    previous_query.set_less_than("id", users.first().unwrap().id.to_string());
    let previous_result = QueryBuilder::generate(Entity::find(), previous_query)
        .all(db).await
        .expect("Cannot find users");

    let previous = if !previous_result.is_empty() {
        previous_result.last().unwrap().id
    } else {
        0
    };

    Ok(QueryResult {
        meta: MetaListData {
            timestamp: Utc::now(),
            count: total_count,
            page: QueryBuilder::current_page(total_count, remaining_count, query_result.limit),
            page_count: QueryBuilder::page_count(total_count, query_result.limit),
            limit: query_result.limit,
            cursor: "id".to_string(),
            next: next as u64,
            previous: previous as u64,
        },
        data: users,
    })
}