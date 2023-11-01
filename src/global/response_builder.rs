use axum::Json;
use http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::global::error_handling::{ErrorDetails, ErrorDetailsDto};

#[derive(Serialize, Deserialize)]
pub struct DataListResponse<T> {
    pub meta: MetaListData,
    pub errors: Vec<ErrorDetails>,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct DataListResponseDto<T> {
    pub meta: MetaListData,
    pub errors: Vec<ErrorDetailsDto>,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub meta: MetaData,
    pub errors: Vec<ErrorDetails>,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetaListData {
    pub timestamp: String,
    pub count: u64,
    pub page: u64,
    pub page_count: u64,
    pub limit: u64,
    pub cursor: String,
    pub next: u64,
    pub previous: u64,
}

impl<T> DataListResponse<T> {
    pub async fn init<E: EntityTrait>(db: &DatabaseConnection, data: Vec<T>) -> DataListResponse<T> {
        // let count = E::find()
        //     .count(&db)
        //     .await
        //     .expect("handle errors properly");

        Self {
            meta: MetaListData {
                timestamp: "".to_string(),
                count: 0,
                page: 0,
                page_count: 0,
                limit: 0,
                cursor: "".to_string(),
                next: 0,
                previous: 0,
            },
            errors: Vec::new(),
            data,
        }
    }

    pub fn respond(&self) -> Result<Json<DataListResponse<T>>, (StatusCode, Json<DataListResponse<T>>)> {
        if !Self.errors.is_empty() {
            match Self.errors.first() {
                None => {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json::from(Self)))
                }
                Some(error) => {
                    Err((error.status_code, Json::from(Self)))
                }
            }
        } else {
            Ok(Json::from(Self))
        }
    }
}

// TODO: fix me
// let count = user::Entity::find()
//     .count(&state.db)
//     .await
//     .expect("Cannot count users");
// let remaining_count = user::Entity::find()
//     .order_by_asc(user::Column::Id)
//     .filter(user::Column::Id.gte(400))
//     .count(&state.db)
//     .await
//     .expect("Cannot find users");
// let page_count = count / limit;
//
// let next = user::Entity::find()
//     .order_by_asc(user::Column::Id)
//     .limit(1)
//     .cursor_by(user::Column::Id)
//     .after(users.last().unwrap().id)
//     .all(&state.db).await
//     .expect("Cannot find users");
// let next_id = if !next.is_empty() {
//     next.first().unwrap().id
// } else {
//     0
// };
//
// let previous = user::Entity::find()
//     .order_by_desc(user::Column::Id)
//     .limit(1)
//     .cursor_by(user::Column::Id)
//     .before(users.first().unwrap().id).all(&state.db).await
//     .expect("Cannot find users");
// let previous_id = if !previous.is_empty() {
//     previous.first().unwrap().id
// } else {
//     0
// };
//
// let mut response = DataListResponse {
//     meta: MetaListData {
//         timestamp: Utc::now().to_string(),
//         count,
//         page: ((count - remaining_count) / limit) + 1,
//         page_count,
//         limit,
//         cursor: "id".to_string(),
//         next: next_id as u64,
//         previous: previous_id as u64,
//     },
//     errors: vec![],
//     data: users,
// };
//
// response.errors.push(ErrorDetailsDto {
//     status_code: 400,
//     error: "Bad Request".to_string(),
//     message: "wtf".to_string(),
// });