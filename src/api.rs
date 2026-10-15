use crate::{
    handlers::{
        auth_handlers, comment_handlers, commentable_handlers, hello_handler, like_handlers,
        post_handlers, project_handlers, tag_handlers, user_handlers,
    },
    middlewares,
};
use actix_web::{
    middleware::from_fn,
    web::{ServiceConfig, delete, get, patch, post, scope},
};

pub fn config(config: &mut ServiceConfig) {
    config.route("/", get().to(hello_handler::hello)).service(
        scope("api/v2")
            .service(
                scope("users")
                    .route("register", post().to(auth_handlers::register))
                    .route("login", post().to(auth_handlers::login))
                    .route("{user_id}", get().to(user_handlers::show)),
            )
            .service(
                scope("auth")
                    .wrap(from_fn(middlewares::auth::auth))
                    .service(
                        scope("users")
                            .route("logout", delete().to(auth_handlers::logout))
                            .route("connected", get().to(auth_handlers::connected))
                            .route("", patch().to(user_handlers::update))
                            .route("", delete().to(user_handlers::destroy)),
                    )
                    .service(
                        scope("verified")
                            .wrap(from_fn(middlewares::verified::verified))
                            .service(
                                scope("comments")
                                    .route(
                                        "{commentable_type}/{commentable_id}",
                                        post().to(commentable_handlers::store),
                                    )
                                    .route("{comment_id}", post().to(comment_handlers::store))
                                    .route("{comment_id}", delete().to(comment_handlers::destroy)),
                            )
                            .service(scope("likes").route(
                                "{likeable_type}/{likeable_id}",
                                post().to(like_handlers::like_or_unlike),
                            )),
                    ),
            )
            .service(
                scope("tags")
                    // .route("", get().to(tag_handlers::index))
                    .route("posts", get().to(tag_handlers::index_posts))
                    .route("projects", get().to(tag_handlers::index_projects))
                    // a effacer
                    .route("all", get().to(tag_handlers::all))
                    .route("{slug}", get().to(tag_handlers::show))
                    .route("{slug}/projects", get().to(tag_handlers::get_projects))
                    .route("{slug}/posts", get().to(tag_handlers::get_posts)),
            )
            .service(
                scope("projects")
                    .route("", get().to(project_handlers::index))
                    .route("pinned", get().to(project_handlers::pinned))
                    .route("{slug}", get().to(project_handlers::show))
                    .route("{project_id}/metrics", get().to(project_handlers::metrics))
                    .route("{project_id}/tags", get().to(project_handlers::get_tags))
                    .route(
                        "{project_id}/adjacent",
                        get().to(project_handlers::adjacent),
                    ),
            )
            .service(
                scope("posts")
                    .route("", get().to(post_handlers::index))
                    .route("pinned", get().to(post_handlers::pinned))
                    .route("{slug}", get().to(post_handlers::show))
                    .route("{post_id}/metrics", get().to(post_handlers::metrics))
                    .route("{post_id}/tags", get().to(post_handlers::get_tags))
                    .route("{post_id}/adjacent", get().to(post_handlers::adjacent)),
            )
            .service(
                scope("comments")
                    .route("{comment_id}", get().to(comment_handlers::index))
                    .route(
                        "{comment_id}/meta",
                        get().to(comment_handlers::get_comment_meta),
                    )
                    .route(
                        "{commentable_type}/{commentable_id}",
                        get().to(commentable_handlers::index),
                    ),
            ),
    );
}
