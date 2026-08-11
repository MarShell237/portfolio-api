use crate::handlers::{hello_handler, project_handlers, tag_handlers};
use actix_web::web::{ServiceConfig, get, scope};

pub fn config(config: &mut ServiceConfig) {
    config.route("/", get().to(hello_handler::hello)).service(
        scope("api/v2")
            .service(
                scope("tags")
                    .route("", get().to(tag_handlers::index))
                    .route("{slug}", get().to(tag_handlers::show)),
            )
            .service(
                scope("projects")
                    .route("", get().to(project_handlers::index))
                    .route("pinned", get().to(project_handlers::pinned))
                    .route("{slug}", get().to(project_handlers::show))
                    .route("{project_id}/metrics", get().to(project_handlers::metrics))
                    .route(
                        "{project_id}/adjacent",
                        get().to(project_handlers::adjacent),
                    ),
            ),
    );
}
