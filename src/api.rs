use crate::handlers::{tag_handlers, test_handlers};
use actix_web::web::{ServiceConfig, get, scope};

pub fn config(config: &mut ServiceConfig) {
    config.route("/", get().to(test_handlers::hello)).service(
        scope("api/v2").service(
            scope("tags")
                .route("", get().to(tag_handlers::index))
                .route("{slug}", get().to(tag_handlers::show)),
        ),
    );
}
