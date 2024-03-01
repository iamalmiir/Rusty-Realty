use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::database::realtor;

#[derive(Debug, serde::Deserialize)]
pub struct Realtor {
    full_name: String,
    email: String,
    photo: Option<String>,
    phone: String,
    is_mvp: Option<bool>,
    description: Option<String>,
}

impl Realtor {
    pub async fn add_realtor(
        db: &DatabaseConnection,
        realtor: Realtor,
    ) -> Result<realtor::Model, DbErr> {
        let new_realtor = realtor::ActiveModel {
            id: Set(nanoid::nanoid!()),
            full_name: Set(realtor.full_name),
            email: Set(realtor.email),
            photo: Set(realtor.photo),
            phone: Set(realtor.phone),
            is_mvp: Set(realtor.is_mvp),
            description: Set(realtor.description),
            ..Default::default()
        };

        match new_realtor.insert(db).await {
            Ok(realtor) => Ok(realtor),
            Err(err) => Err(err),
        }
    }
    /// TODO: Add pagination
    pub async fn fetch_all(db: &DatabaseConnection) -> Result<Vec<realtor::Model>, DbErr> {
        match realtor::Entity::find().all(db).await {
            Ok(realtors) => Ok(realtors),

            Err(err) => Err(err),
        }
    }
}
