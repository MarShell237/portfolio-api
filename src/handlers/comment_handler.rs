use actix_identity::Identity;
use actix_web::{
    Responder,
    web::{Data, Path, Query},
};

use crate::{
    dtos::responses::comment_response::CommentResponse,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState, pagination_meta::PaginationParams},
    repositories::comment_repositories,
};

pub async fn index(
    comment_id: Path<i64>,
    query: Query<PaginationParams>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let (comments, meta) = comment_repositories::get_children_of_comments(
        *comment_id,
        query.page.unwrap_or(1),
        &app_state.db_pool,
    )
    .await?;
    let comments_response: Vec<CommentResponse> =
        comments.into_iter().map(CommentResponse::from).collect();
    Ok(ApiResponse::ok_with_pagination(
        "Commentaires enfants récupérés avec succès.",
        Some(comments_response),
        meta,
    ))
}

pub async fn get_comment_meta(
    comment_id: Path<i64>,
    session: Option<Identity>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let user_id = session
        .and_then(|identity| identity.id().ok())
        .and_then(|id_str| id_str.parse::<i64>().ok())
        .unwrap_or(-1);
    let comment_meta = comment_repositories::get_comment_meta(
        comment_id.into_inner(),
        user_id,
        &app_state.db_pool,
    )
    .await?;
    Ok(ApiResponse::ok(
        "métadonnées du commentaire récupérées avec succès",
        Some(comment_meta),
    ))
}
