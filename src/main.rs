use actix_web::{App, HttpServer, middleware::Logger};
use sea_orm::{Database, DatabaseConnection};

mod handlers;
mod helpers;

use helpers::config::{APP_PORT, APP_URL, DATABASE_URL};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let db: DatabaseConnection = Database::connect(&*DATABASE_URL)
        .await
        .expect("Database connection failed");
    HttpServer::new(|| {
        App::new()
            .configure(handlers::api::config)
            .wrap(Logger::default())
    })
    .bind((APP_URL.as_str(), *APP_PORT))?
    .run()
    .await
}
