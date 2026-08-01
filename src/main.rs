use actix_web::{App, HttpServer, middleware::Logger, web};
use sea_orm::{Database, DatabaseConnection};

mod api;
mod config;
mod handlers;
mod helpers;

use config::{APP_PORT, APP_URL, DATABASE_URL};
use helpers::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let db: DatabaseConnection = Database::connect(&*DATABASE_URL)
        .await
        .expect("Database connection failed");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .configure(api::config)
            .wrap(Logger::default())
    })
    .bind((APP_URL.as_str(), *APP_PORT))
    .expect("failled to load http server")
    .run()
    .await
}
