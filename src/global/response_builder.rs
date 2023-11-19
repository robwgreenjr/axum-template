use axum::Json;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::database::query_builder::QueryResult;
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

#[derive(Clone)]
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

    fn default() -> Self {
        MetaListData {
            timestamp: Utc::now(),
            count: 0,
            page: 0,
            page_count: 0,
            limit: 0,
            cursor: "".to_string(),
            next: 0,
            previous: 0,
        }
    }
}

impl<T> DataListResponse<T> {
    pub async fn init(
        result: Option<QueryResult<T>>,
        errors: Option<Vec<ErrorDetails>>,
    ) -> DataListResponse<T>
    {
        Self {
            meta: result.as_ref().map_or_else(
                || MetaListData::default(),
                |result| result.meta.clone(),
            ),
            errors: errors.unwrap_or_default(),
            data: result.map_or_else(
                || vec![],
                |result| result.data,
            ),
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