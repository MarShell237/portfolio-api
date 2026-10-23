use actix_multipart::form::MultipartForm;
use actix_web::{Responder, web::Data};

use crate::{
    dtos::{forms::message_form::MessageForm, responses::message_response::MessageResponse},
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::message_repositories,
};

pub async fn store(
    request: MultipartForm<MessageForm>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let (message_request, attachment) = request.into_inner().validate_and_split()?;
    let message = message_repositories::create(message_request, &app_state.db_pool).await?;
    let message_response = MessageResponse::from(message);
    Ok(ApiResponse::created(
        "Le message a été envoyé aux administrateurs avec succès",
        Some(message_response),
    ))
}
