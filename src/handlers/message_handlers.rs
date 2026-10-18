use actix_web::{Responder, web::Data};
use actix_web_validator::Json;

use crate::{
    dtos::{
        requests::message_request::MessageRequest, responses::message_response::MessageResponse,
    },
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::message_repositories,
};

pub async fn store(
    Json(message_request): Json<MessageRequest>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let message = message_repositories::create(message_request, &app_state.db_pool).await?;
    let message_response = MessageResponse::from(message);
    Ok(ApiResponse::created(
        "Le message a été envoyé aux administrateurs avec succès",
        Some(message_response),
    ))
}
