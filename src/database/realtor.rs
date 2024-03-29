//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde_derive::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "realtor")]
pub struct Model {
    #[sea_orm(primary_key, unique, auto_increment = false)]
    pub id: String,
    pub full_name: String,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub photo: Option<String>,
    pub phone: String,
    pub is_mvp: Option<bool>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
