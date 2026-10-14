use actix_identity::Identity;
use actix_web::{
    Responder,
    web::{Data, Path, Query, ReqData},
};
use actix_web_validator::Json;

use crate::{
    dtos::{
        requests::comment_request::CommentRequest, responses::comment_response::CommentResponse,
    },
    enums::user_role::UserRole,
    errors::AppError,
    helpers::{api_response::ApiResponse, app_state::AppState, pagination_meta::PaginationParams},
    repositories::{comment_repositories, user_repositories},
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

pub async fn store(
    Json(request): Json<CommentRequest>,
    comment_id: Path<i64>,
    user_id: ReqData<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let admin = user_repositories::get_first_admin(&app_state.db_pool).await?;
    let comment = comment_repositories::create_sub_comment(
        *user_id,
        *comment_id,
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

pub async fn destroy(
    comment_id: Path<i64>,
    user_id: ReqData<i64>,
    app_state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let comment = comment_repositories::find_or_fail(*comment_id, &app_state.db_pool).await?;
    if !comment.commenter_id != *user_id
        && !user_repositories::has_role(*user_id, UserRole::ADMIN, &app_state.db_pool).await?
    {
        return Err(AppError::forbidden(
            "Vous n'etes pas le createur de ce commentaire",
        ));
    }
    comment_repositories::delete(comment, &app_state.db_pool).await?;
    Ok(ApiResponse::ok(
        "Le commentaire a ete suprimmer avec succes",
        None::<()>,
    ))
}
