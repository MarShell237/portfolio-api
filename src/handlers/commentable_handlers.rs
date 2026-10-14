use actix_web::{
    Responder,
    web::{Data, Path, Query, ReqData},
};
use actix_web_validator::Json;

use crate::{
    dtos::{
        requests::comment_request::CommentRequest, responses::comment_response::CommentResponse,
    },
    enums::commentable_enum::CommentableEnum,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState, pagination_meta::PaginationParams},
    repositories::{comment_repositories, user_repositories},
};

pub async fn index(
    path: Path<(String, i64)>,
    query: Query<PaginationParams>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let (commentable_type, commentable_id) = path.into_inner();
    CommentableEnum::validate_commentable(
        commentable_type.as_str(),
        commentable_id,
        &app_state.db_pool,
    )
    .await?;
    let (comments, meta) = comment_repositories::get_comments_of_commentable(
        commentable_type,
        commentable_id,
        query.page.unwrap_or(1),
        &app_state.db_pool,
    )
    .await?;
    let comments_response: Vec<CommentResponse> =
        comments.into_iter().map(CommentResponse::from).collect();
    Ok(ApiResponse::ok_with_pagination(
        "Commentaires racines récupérés avec succès.",
        Some(comments_response),
        meta,
    ))
}

pub async fn store(
    Json(request): Json<CommentRequest>,
    path: Path<(String, i64)>,
    user_id: ReqData<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let (commentable_type, commentable_id) = path.into_inner();
    CommentableEnum::validate_commentable(
        commentable_type.as_str(),
        commentable_id,
        &app_state.db_pool,
    )
    .await?;

    let admin = user_repositories::get_first_admin(&app_state.db_pool).await?;
    let comment = comment_repositories::create(
        *user_id,
        commentable_type,
        commentable_id,
        request.content,
        &app_state.db_pool,
    )
    .await?;
    let comment_response = CommentResponse::from(comment);
    Ok(ApiResponse::ok(
        "Votre commentaire a été ajouté avec succès !",
        Some(comment_response),
    ))
}
