use actix_web::Responder;

use crate::helpers::api_response::ApiResponse;

pub async fn hello() -> impl Responder {
    ApiResponse::ok(
        "Hello, welcome to my portfolio API in Actix-web",
        None::<()>,
    )
}
