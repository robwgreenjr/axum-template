use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorDetailsDto {
    pub status_code: u16,
    pub error: String,
    pub message: String,
}

pub struct ErrorDetails {
    pub status_code: StatusCode,
    pub message: String,
}

impl ErrorDetails {
    pub fn to_dto(&self) -> ErrorDetailsDto {
        ErrorDetailsDto {
            status_code: self.status_code.as_u16(),
            error: self.status_code.as_str().to_string(),
            message: self.message.clone(),
        }
    }
}