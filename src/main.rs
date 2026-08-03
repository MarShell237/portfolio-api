mod api;
mod config;
mod dtos;
mod handlers;
mod helpers;

use actix_web::{App, HttpServer, middleware::Logger, web};
use config::{APP_PORT, APP_URL, DATABASE_URL};
use helpers::app_state::AppState;
use sea_orm::{Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let db_pool: DatabaseConnection = Database::connect(&*DATABASE_URL)
        .await
        .expect("Database connection failed");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
            }))
            .configure(api::config)
            .wrap(Logger::default())
    })
    .bind((APP_URL.as_str(), *APP_PORT))
    .expect("failled to load http server")
    .run()
    .await
}
