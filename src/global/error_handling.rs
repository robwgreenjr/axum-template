use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorDetailsDto {
    pub status_code: u16,
    pub error: String,
    pub message: String,
}

pub struct ErrorDetails {
    status_code: StatusCode,
    message: String,
}