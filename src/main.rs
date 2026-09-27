mod api;
mod config;
mod dtos;
mod enums;
mod errors;
mod handlers;
mod helpers;
mod repositories;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{
    SessionMiddleware,
    config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy},
    storage::RedisSessionStore,
};
use actix_web::{
    App, HttpServer,
    cookie::{Key, SameSite, time::Duration},
    middleware::{Compress, Logger},
    web,
};
use config::{
    ALLOWED_ORIGIN, APP_PORT, APP_URL, COOKIE_DOMAIN, COOKIE_NAME, COOKIE_SECURE, DATABASE_URL,
    REDIS_URL,
};
use helpers::app_state::AppState;
use migrations::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool: DatabaseConnection = Database::connect(&*DATABASE_URL).await.unwrap();

    let key = Key::generate();
    let redis = RedisSessionStore::new(&*REDIS_URL).await.unwrap();
    Migrator::up(&db_pool, None).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&*ALLOWED_ORIGIN)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
            }))
            .configure(api::config)
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(redis.clone(), key.clone())
                    .cookie_name(COOKIE_NAME.clone())
                    .cookie_domain(COOKIE_DOMAIN.clone())
                    .cookie_secure(*COOKIE_SECURE)
                    .cookie_same_site(SameSite::Lax)
                    .cookie_http_only(true)
                    .cookie_content_security(CookieContentSecurity::Signed)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest)
                            .session_ttl(Duration::days(7)),
                    )
                    .build(),
            )
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind((APP_URL.as_str(), *APP_PORT))
    .unwrap()
    .run()
    .await
}
