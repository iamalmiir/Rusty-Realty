use actix_web::{web, App, HttpServer};

mod actors;
mod database;
mod services;

use dotenv::dotenv;
use sea_orm::Database;
use services::realtor::{create_realtor, get_realtors};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = Database::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(get_realtors)
            .service(create_realtor)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
