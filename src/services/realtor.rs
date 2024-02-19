use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::database::realtor;

async fn fetch_realtor(database: &DatabaseConnection) -> Result<Vec<realtor::Model>, DbErr> {
    match realtor::Entity::find().all(database).await {
        Ok(realtors) => Ok(realtors),

        Err(err) => Err(err),
    }
}

#[derive(Debug, serde::Deserialize)]
struct Realtor {
    full_name: String,
    email: String,
    photo: Option<String>,
    phone: String,
    is_mvp: Option<bool>,
    description: Option<String>,
}

async fn add_realtor(
    database: &DatabaseConnection,
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

    match new_realtor.insert(database).await {
        Ok(realtor) => Ok(realtor),
        Err(err) => Err(err),
    }
}

/// Fetches all realtors from the database.
///
/// # Arguments
///
/// * `database` - The database connection.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `realtor::Model` if successful, or a `DbErr` if an error occurs.
#[get("/realtors/all")]
pub async fn get_realtor(db: web::Data<DatabaseConnection>) -> impl Responder {
    match fetch_realtor(db.as_ref()).await {
        Ok(realtors) => HttpResponse::Ok().json(realtors),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Adds a new realtor to the database.
///
/// This function is an endpoint for a POST request at the path "/realtors/add".
///
/// # Arguments
///
/// * `db` - A shared reference to a `DatabaseConnection` wrapped in `web::Data`. This is the database connection.
/// * `form` - A JSON payload that is expected to be of the `Realtor` type, wrapped in `web::Json`. This is the data of the realtor to be added.
///
/// # Returns
///
/// Returns an object that implements the `Responder` trait. If the realtor is successfully added to the database, it returns an HTTP response with a status code of 200 (OK) and the newly created realtor in the response body as JSON. If there is an error adding the realtor to the database, it returns an HTTP response with a status code of 500 (Internal Server Error).
///
/// # Errors
///
/// Returns `HttpResponse::InternalServerError` if there is an error adding the realtor to the database.
#[post("/realtors/add")]
pub async fn create_realtor(
    db: web::Data<DatabaseConnection>,
    form: web::Json<Realtor>,
) -> impl Responder {
    match add_realtor(db.as_ref(), form.into_inner()).await {
        Ok(realtor) => HttpResponse::Ok().json(realtor),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
