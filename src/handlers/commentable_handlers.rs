use actix_identity::Identity;
use actix_web::{
    Responder,
    web::{Data, Path},
};
use actix_web_validator::Json;

use crate::{
    dtos::requests::comment_request::CommentRequest,
    enums::commentable_enum::CommentableEnum,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::user_repositories,
};

pub async fn store(
    Json(_request): Json<CommentRequest>,
    commentable_type: Path<String>,
    commentable_id: Path<i64>,
    identity: Option<Identity>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    CommentableEnum::validate_commentable(
        commentable_type.into_inner().as_str(),
        commentable_id.into_inner(),
        &app_state.db_pool,
    )
    .await?;

    let _user_id = identity
        .and_then(|identity| identity.id().ok())
        .and_then(|id_str| id_str.parse::<i64>().ok());

    let _admin = user_repositories::get_first_admin(&app_state.db_pool).await?;
    Ok(ApiResponse::ok("Commentaire créé", None::<()>))
}
