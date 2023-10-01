use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub created_on: String,
    pub updated_on: String,
}