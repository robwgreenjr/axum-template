use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dto {
    pub id: Option<u64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
}

pub struct Index {
    pub id: Option<u64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_on: Option<DateTimeWithTimeZone>,
    pub updated_on: Option<DateTimeWithTimeZone>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_base")]
pub struct Model {
    #[sea_orm(unique)]
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub email: String,
    #[sea_orm(unique)]
    pub phone: Option<String>,
    pub created_on: Option<DateTimeWithTimeZone>,
    pub updated_on: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
