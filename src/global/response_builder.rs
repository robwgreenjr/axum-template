use serde::{Deserialize, Serialize};

use crate::global::error_handling::ErrorDetailsDto;

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub meta: MetaData,
    pub errors: Vec<ErrorDetailsDto>,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub timestamp: String,
    pub count: u64,
    pub page: u64,
    pub page_count: u64,
    pub limit: u64,
    pub cursor: String,
    pub next: u64,
    pub previous: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MetaQueryData {}