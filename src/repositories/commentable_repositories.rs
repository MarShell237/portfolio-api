use actix_identity::Identity;
use actix_web::{Responder, dev::Path, web::Data};
use actix_web_validator::Json;

use crate::{
    dtos::requests::comment_request::CommentRequest, errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
};

pub async fn store(
    Json(_request): Json<CommentRequest>,
    commentable_type: Path<String>,
    commentable_id: Path<i64>,
    identity: Option<Identity>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let _user_id: Option<i64> = identity
        .and_then(|identity| identity.id().ok())
        .and_then(|id_str| id_str.parse::<i64>().ok());
    Ok(ApiResponse::ok("Commentaire créé", None::<()>))
}
