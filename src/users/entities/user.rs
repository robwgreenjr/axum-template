use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_simple")]
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
