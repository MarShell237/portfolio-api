use crate::handlers::{tag_handlers, test_handlers};
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("api/v2")
            .route("", web::get().to(test_handlers::hello))
            .route("manual", web::get().to(test_handlers::manual_hello))
            .route("echo", web::post().to(test_handlers::echo))
            .service(web::scope("tags").route("", web::get().to(tag_handlers::index))),
    );
}
