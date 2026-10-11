use actix_web::{
    Responder,
    web::{Data, Path, ReqData},
};
use actix_web_validator::Json;

use crate::{
    dtos::{
        requests::comment_request::CommentRequest, responses::comment_response::CommentResponse,
    },
    enums::commentable_enum::CommentableEnum,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState},
    repositories::{comment_repositories, user_repositories},
};

pub async fn store(
    Json(request): Json<CommentRequest>,
    commentable_type: Path<String>,
    commentable_id: Path<i64>,
    user_id: ReqData<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    CommentableEnum::validate_commentable(
        commentable_type.as_str(),
        *commentable_id,
        &app_state.db_pool,
    )
    .await?;

    let _admin = user_repositories::get_first_admin(&app_state.db_pool).await?;
    let comment = comment_repositories::create(
        *user_id,
        commentable_type.into_inner(),
        commentable_id.into_inner(),
        request.content,
        &app_state.db_pool,
    )
    .await?;
    let comment_response = CommentResponse::from(comment);
    Ok(ApiResponse::ok("Commentaire créé", Some(comment_response)))
}
