use axum::Json;
use chrono::{DateTime, Utc};
use http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait};
use serde::{Deserialize, Serialize};

use crate::global::error_handling::{ErrorDetails, ErrorDetailsDto};

#[derive(Serialize, Deserialize)]
pub struct DataListResponseDto<T> {
    pub meta: MetaListDataDto,
    pub errors: Vec<ErrorDetailsDto>,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct DataResponseDto<T> {
    pub meta: MetaDataDto,
    pub errors: Vec<ErrorDetailsDto>,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaDataDto {
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetaListDataDto {
    pub timestamp: String,
    pub count: u64,
    pub page: u64,
    pub page_count: u64,
    pub limit: u64,
    pub cursor: String,
    pub next: u64,
    pub previous: u64,
}

pub struct DataListResponse<T> {
    pub meta: MetaListData,
    pub errors: Vec<ErrorDetails>,
    pub data: Vec<T>,
}

pub struct DataResponse<T> {
    pub meta: MetaData,
    pub errors: Vec<ErrorDetails>,
    pub data: Vec<T>,
}

pub struct MetaData {
    pub timestamp: DateTime<Utc>,
}

pub struct MetaListData {
    pub timestamp: DateTime<Utc>,
    pub count: u64,
    pub page: u64,
    pub page_count: u64,
    pub limit: u64,
    pub cursor: String,
    pub next: u64,
    pub previous: u64,
}

impl MetaData {
    fn to_dto(&self) -> MetaDataDto {
        MetaDataDto {
            timestamp: self.timestamp.to_string(),
        }
    }
}

impl MetaListData {
    fn to_dto(&self) -> MetaListDataDto {
        MetaListDataDto {
            timestamp: self.timestamp.to_string(),
            count: self.count,
            page: self.page,
            page_count: self.page_count,
            limit: self.limit,
            cursor: self.cursor.clone(),
            next: self.next,
            previous: self.previous,
        }
    }
}

impl<T> DataListResponse<T> {
    pub async fn init<E, M>(
        db: &DatabaseConnection,
        data: Option<Vec<T>>,
        errors: Option<Vec<ErrorDetails>>,
    ) -> DataListResponse<T>
        where
            E: EntityTrait<Model=M>,
            M: ModelTrait,
    {
        let count = E::find()
            .count(db)
            .await
            .expect("Error counting");

        Self {
            meta: MetaListData {
                timestamp: Utc::now(),
                count: 0,
                page: 0,
                page_count: 0,
                limit: 0,
                cursor: "".to_string(),
                next: 0,
                previous: 0,
            },
            errors: match errors {
                None => {
                    vec![]
                }
                Some(errors) => {
                    errors
                }
            },
            data: match data {
                None => {
                    vec![]
                }
                Some(data) => {
                    data
                }
            },
        }
    }

    pub fn respond(self) -> Result<Json<DataListResponseDto<T>>, (StatusCode, Json<DataListResponseDto<T>>)> {
        if !self.errors.is_empty() {
            match self.errors.first() {
                None => {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json::from(self.to_dto())))
                }
                Some(error) => {
                    Err((error.status_code, Json::from(self.to_dto())))
                }
            }
        } else {
            Ok(Json::from(self.to_dto()))
        }
    }

    fn to_dto(self) -> DataListResponseDto<T> {
        DataListResponseDto {
            meta: self.meta.to_dto(),
            errors: self.errors.into_iter().map(|error| error.to_dto()).collect(),
            data: self.data,
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