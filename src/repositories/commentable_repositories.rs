use actix_identity::Identity;
use actix_web::{Responder, dev::Path, web::Data};
use actix_web_validator::Json;

use crate::{
    dtos::requests::comment_request::CommentRequest, errors::AppError, helpers::app_state::AppState,
};

pub async fn store(
    Json(request): Json<CommentRequest>,
    commentable_type: Path<String>,
    commentable_id: Path<i64>,
    identity: Option<Identity>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let user_id = identity.and_then(|identity| identity.id().ok());
    todo!()
}
