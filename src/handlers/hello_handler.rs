use actix_web::Responder;
use sea_orm::sea_query::prelude::Utc;

use crate::helpers::api_response::ApiResponse;

pub async fn hello() -> impl Responder {
    let human_readable_time = Utc::now().format("%d %m %Y %H:%M:%S").to_string();

    ApiResponse::ok(
        "Hello, welcome to my portfolio API in Actix-web",
        Some(human_readable_time),
    )
}
